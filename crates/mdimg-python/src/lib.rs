use mdimg::{Image, Kind, Link, Options};
use pyo3::prelude::*;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[pyclass(frozen, eq, hash, module = "mdimg", name = "Link")]
#[derive(PartialEq, Eq, Hash)]
enum PyLink {
    Inline {},
    Reference { id: String },
    Collapsed { id: String },
    Shortcut { id: String },
    Wiki { has_pothole: bool },
}
#[pymethods]
impl PyLink {
    fn __repr__(&self) -> String {
        match self {
            PyLink::Inline {} => "Link.Inline()".to_string(),
            PyLink::Reference { id } => format!("Link.Reference(id={id:?})"),
            PyLink::Collapsed { id } => format!("Link.Collapsed(id={id:?})"),
            PyLink::Shortcut { id } => format!("Link.Shortcut(id={id:?})"),
            PyLink::Wiki { has_pothole } => {
                format!("Link.Wiki(has_pothole={})", bool_repr(*has_pothole))
            }
        }
    }
}

#[pyclass(frozen, module = "mdimg", name = "MarkdownImage")]
struct PyMarkdownImage {
    #[pyo3(get)]
    url: String,
    #[pyo3(get)]
    title: String,
    #[pyo3(get)]
    alt: String,
    #[pyo3(get)]
    link: Py<PyAny>,
}
#[pymethods]
impl PyMarkdownImage {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "MarkdownImage(url={:?}, title={:?}, alt={:?}, link={})",
            self.url,
            self.title,
            self.alt,
            self.link.bind(py).repr()?
        ))
    }
}

#[pyclass(frozen, module = "mdimg", name = "HtmlImage")]
struct PyHtmlImage {
    #[pyo3(get)]
    attributes: BTreeMap<String, String>,
    #[pyo3(get)]
    self_closing: bool,
}
#[pymethods]
impl PyHtmlImage {
    fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    fn __repr__(&self) -> String {
        format!(
            "HtmlImage(attributes={:?}, self_closing={})",
            self.attributes,
            bool_repr(self.self_closing)
        )
    }
}

fn bool_repr(value: bool) -> &'static str {
    if value { "True" } else { "False" }
}

#[pyclass(frozen, module = "mdimg", name = "Image")]
struct PyImage {
    #[pyo3(get)]
    span: (usize, usize),
    #[pyo3(get)]
    raw: String,
    #[pyo3(get)]
    url: Option<String>,
    #[pyo3(get)]
    alt: Option<String>,
    #[pyo3(get)]
    kind: Py<PyAny>,
}
#[pymethods]
impl PyImage {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "Image(span={:?}, raw={:?}, kind={})",
            self.span,
            self.raw,
            self.kind.bind(py).repr()?
        ))
    }
}

struct Spans<'a> {
    text: &'a str,
    byte: usize,
    chars: usize,
}
impl<'a> Spans<'a> {
    fn new(text: &'a str) -> Self {
        Self { text, byte: 0, chars: 0 }
    }

    fn at(&mut self, byte: usize) -> usize {
        self.chars += self.text[self.byte..byte].chars().count();
        self.byte = byte;
        self.chars
    }
}

fn link(py: Python<'_>, link: &Link<'_>) -> PyResult<Py<PyAny>> {
    let variant = match link {
        Link::Inline => PyLink::Inline {},
        Link::Reference { id } => PyLink::Reference { id: id.to_string() },
        Link::Collapsed { id } => PyLink::Collapsed { id: id.to_string() },
        Link::Shortcut { id } => PyLink::Shortcut { id: id.to_string() },
        Link::Wiki { has_pothole } => PyLink::Wiki { has_pothole: *has_pothole },
    };
    Ok(variant.into_pyobject(py)?.into_any().unbind())
}

fn kind(py: Python<'_>, kind: &Kind<'_>) -> PyResult<Py<PyAny>> {
    match kind {
        Kind::Markdown(image) => Py::new(
            py,
            PyMarkdownImage {
                url: image.url.to_string(),
                title: image.title.to_string(),
                alt: image.alt.to_string(),
                link: link(py, &image.link)?,
            },
        )
        .map(Py::into_any),
        Kind::Html(image) => Py::new(
            py,
            PyHtmlImage {
                attributes: image
                    .attributes()
                    .map(|(name, value)| (name.to_string(), value.to_string()))
                    .collect(),
                self_closing: image.self_closing(),
            },
        )
        .map(Py::into_any),
    }
}

fn image(py: Python<'_>, image: &Image<'_>, spans: &mut Spans<'_>) -> PyResult<Py<PyImage>> {
    let kind = kind(py, &image.kind)?;
    let span = (spans.at(image.span.start), spans.at(image.span.end));
    Py::new(
        py,
        PyImage {
            span,
            raw: image.raw.to_string(),
            url: image.url().map(str::to_string),
            alt: image.alt().map(str::to_string),
            kind,
        },
    )
}

#[pyfunction]
#[pyo3(signature = (text, map_fn, *, markdown = 0, html = true))]
fn map_images(
    py: Python<'_>,
    text: &str,
    map_fn: &Bound<'_, PyAny>,
    markdown: i64,
    html: bool,
) -> PyResult<String> {
    let bits = mdimg::pulldown_cmark::Options::from_bits_truncate(markdown as u32);
    let options = Options { markdown: bits, html };
    let mut spans = Spans::new(text);
    let mut failure: Option<PyErr> = None;

    let output = mdimg::map_images_with(text, options, |source| {
        if failure.is_some() {
            return Cow::Borrowed(source.raw);
        }
        let mapped = image(py, &source, &mut spans)
            .and_then(|argument| map_fn.call1((argument,))?.extract::<String>());
        match mapped {
            Ok(mapped) => Cow::Owned(mapped),
            Err(error) => {
                failure = Some(error);
                Cow::Borrowed(source.raw)
            }
        }
    });

    match failure {
        Some(error) => Err(error),
        None => Ok(output),
    }
}

#[pyfunction]
fn markdown_flags() -> BTreeMap<String, u32> {
    mdimg::pulldown_cmark::Options::all()
        .iter_names()
        .map(|(name, flag)| (name.to_string(), flag.bits()))
        .collect()
}

#[pymodule]
fn _mdimg(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyImage>()?;
    module.add_class::<PyMarkdownImage>()?;
    module.add_class::<PyHtmlImage>()?;
    module.add_class::<PyLink>()?;
    module.add_function(wrap_pyfunction!(map_images, module)?)?;
    module.add_function(wrap_pyfunction!(markdown_flags, module)?)?;
    Ok(())
}
