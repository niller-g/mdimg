//! Map images in markdown text across many different syntaxes, including `![alt](url)` and
//! `<img src="url" alt="alt">`. This crate provides a simple way of surgically substituting
//! images in a Markdown document, leaving everything else untouched.
//!
//! The markdown parser is [pulldown-cmark](https://pulldown-cmark.github.io/pulldown-cmark/) and
//! simply re-exports its functionality and configuration options.
//!
//! # Example
//!
//! ```rust
//! let output = mdimg::map_images("Before ![alt](file.png) after", |image| {
//!     format!("[image: alt={} url={}]", image.alt().unwrap_or(""), image.url().unwrap())
//! });
//! assert_eq!(output, "Before [image: alt=alt url=file.png] after");
//! ```
//!
//! # Sources
//!
//! [`Image::url`] gives you the source exactly as written, so you can decide yourself which
//! images to rewrite.

use html5gum::{DefaultEmitter, StartTag, Token, Tokenizer};
use pulldown_cmark::{Event, LinkType, Parser, Tag, TagEnd};
use std::borrow::Cow;
use std::fmt::{self, Write};
use std::hash::Hash;
use std::iter::Peekable;
use std::mem;
use std::ops::Range;

pub use pulldown_cmark::{self, CowStr};

/// A markdown or HTML image.
#[derive(Debug, Clone)]
pub struct Image<'a> {
    /// The span of the image in the original text.
    pub span: Range<usize>,
    /// The raw text of the image. Let your `map_fn` return this to leave the image unchanged.
    pub raw: &'a str,
    /// The kind of image, either markdown or HTML.
    pub kind: Kind<'a>,
}
impl<'a> Image<'a> {
    fn new(span: Range<usize>, raw: &'a str, kind: Kind<'a>) -> Self {
        Self { span, raw, kind }
    }

    /// Returns the URL of the image.
    pub fn url(&self) -> Option<&str> {
        match &self.kind {
            Kind::Markdown(image) => Some(&image.url),
            Kind::Html(image) => image.attribute("src"),
        }
    }

    /// Returns the alternative text of the image.
    pub fn alt(&self) -> Option<&str> {
        match &self.kind {
            Kind::Markdown(image) => Some(&image.alt),
            Kind::Html(image) => image.attribute("alt"),
        }
    }
}

/// The kind of image, either markdown or HTML.
#[derive(Debug, Clone)]
pub enum Kind<'a> {
    Markdown(MarkdownImage<'a>),
    Html(HtmlImage),
}
impl<'a> Kind<'a> {
    fn markdown(
        url: CowStr<'a>,
        title: CowStr<'a>,
        alt: CowStr<'a>,
        link_type: LinkType,
        id: CowStr<'a>,
    ) -> Self {
        Kind::Markdown(MarkdownImage { url, title, alt, link: Link::new(link_type, id) })
    }

    fn html(tag: StartTag<usize>) -> Self {
        Kind::Html(HtmlImage { tag })
    }
}

/// A markdown image.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarkdownImage<'a> {
    pub url: CowStr<'a>,
    pub title: CowStr<'a>,
    pub alt: CowStr<'a>,
    pub link: Link<'a>,
}

/// The type of link used in a markdown image.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Link<'a> {
    /// - Inline link like `[foo](bar)`
    /// - Autolink like `<http://foo.bar/baz>`
    /// - Email address in autolink like `<john@example.org>`
    Inline,
    /// Reference link like `[foo][bar]` where `bar` is the `id`.
    Reference { id: CowStr<'a> },
    /// Collapsed link like `[foo][]` where `foo` is the id.
    Collapsed { id: CowStr<'a> },
    /// Shortcut link like `[foo]` where `foo` is the id.
    Shortcut { id: CowStr<'a> },
    /// Wikilink link like `[[foo]]` or `[[foo|bar]]`
    Wiki {
        /// `true` if the wikilink was piped.
        ///
        /// * `true` - `[[foo|bar]]`
        /// * `false` - `[[foo]]`
        has_pothole: bool,
    },
}
impl<'a> Link<'a> {
    fn new(link_type: LinkType, id: CowStr<'a>) -> Self {
        match link_type {
            LinkType::Reference | LinkType::ReferenceUnknown => Link::Reference { id },
            LinkType::Collapsed | LinkType::CollapsedUnknown => Link::Collapsed { id },
            LinkType::Shortcut | LinkType::ShortcutUnknown => Link::Shortcut { id },
            LinkType::WikiLink { has_pothole } => Link::Wiki { has_pothole },
            LinkType::Inline | LinkType::Autolink | LinkType::Email => Link::Inline,
        }
    }
}

/// An HTML image.
#[derive(Clone)]
pub struct HtmlImage {
    tag: StartTag<usize>,
}
impl HtmlImage {
    /// Returns the value of the given attribute, if it exists. `name` is case-insensitive.
    pub fn attribute(&self, name: &str) -> Option<&str> {
        let (_, attribute) = self
            .tag
            .attributes
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name.as_bytes()))?;
        str::from_utf8(&attribute.value).ok()
    }

    /// Returns an iterator over the attributes of the image.
    pub fn attributes(&self) -> impl Iterator<Item = (&str, &str)> {
        self.tag.attributes.iter().filter_map(|(name, value)| {
            Some((str::from_utf8(name).ok()?, str::from_utf8(&value.value).ok()?))
        })
    }

    /// Returns `true` if the image is self-closing, `false` otherwise.
    pub fn self_closing(&self) -> bool {
        self.tag.self_closing
    }
}
impl fmt::Debug for HtmlImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.attributes()).finish()
    }
}

/// Options for mapping images in markdown text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Options {
    /// Markdown parsing options.
    pub markdown: pulldown_cmark::Options,
    /// If `true`, HTML images will be mapped. If `false`, HTML images will be ignored.
    pub html: bool,
}
impl Default for Options {
    fn default() -> Self {
        Self { markdown: pulldown_cmark::Options::empty(), html: true }
    }
}

/// Map all images in the given markdown `text` using the provided `map_fn`.
///
/// ```rust
/// let output = mdimg::map_images("Before ![alt](file.png) after", |image| {
///     format!("[image: alt={} url={}]", image.alt().unwrap_or(""), image.url().unwrap())
/// });
/// assert_eq!(output, "Before [image: alt=alt url=file.png] after");
/// ```
pub fn map_images<'a, F, R>(text: &'a str, map_fn: F) -> String
where
    F: FnMut(Image<'a>) -> R,
    R: AsRef<str>,
{
    map_images_with(text, Options::default(), map_fn)
}

/// Map all images in the given markdown `text` using the provided `map_fn` with `options`.
pub fn map_images_with<'a, F, R>(text: &'a str, options: Options, map_fn: F) -> String
where
    F: FnMut(Image<'a>) -> R,
    R: AsRef<str>,
{
    let mut result = String::with_capacity(text.len());
    map_images_into(&mut result, text, options, map_fn)
        .expect("writing into a String is infallible");
    result
}

/// Map all images in the given markdown `text` into the provided `out` writer using the provided `map_fn` with `options`.
pub fn map_images_into<'a, W, F, R>(
    out: &mut W,
    text: &'a str,
    options: Options,
    mut map_fn: F,
) -> fmt::Result
where
    W: Write,
    F: FnMut(Image<'a>) -> R,
    R: AsRef<str>,
{
    let mut last = 0;
    let mut html_lines: Vec<Range<usize>> = Vec::new();
    let mut events = Parser::new_ext(text, options.markdown).into_offset_iter().peekable();

    while let Some((event, range)) = events.next() {
        if range.start < last {
            continue;
        }
        match event {
            Event::Start(Tag::Image { link_type, dest_url, title, id }) => {
                out.write_str(&text[last..range.start])?;
                let alt = alt_text(&mut events, range.end);
                let mut span = range;
                if link_type == LinkType::Collapsed && text[span.end..].starts_with("[]") {
                    span.end += 2;
                }
                let raw = &text[span.clone()];
                last = span.end;
                let img =
                    Image::new(span, raw, Kind::markdown(dest_url, title, alt, link_type, id));
                let mapped = map_fn(img);
                out.write_str(mapped.as_ref())?;
            }
            Event::InlineHtml(content) if options.html => {
                if !contains_img(&content) {
                    continue;
                }
                out.write_str(&text[last..range.start])?;
                last = range.end;
                let mut lines = Vec::new();
                let mut pos = range.start;
                for (source_line, content_line) in
                    text[range.clone()].split_inclusive('\n').zip(content.split_inclusive('\n'))
                {
                    let prefix = source_line.len() - content_line.len();
                    lines.push(pos + prefix..pos + source_line.len());
                    pos += source_line.len();
                }
                map_block_images(out, text, range, &lines, &mut map_fn)?;
            }
            Event::Html(_) if options.html => html_lines.push(range),
            Event::End(TagEnd::HtmlBlock) if options.html => {
                out.write_str(&text[last..range.start])?;
                last = range.end;
                let lines = mem::take(&mut html_lines);
                map_block_images(out, text, range, &lines, &mut map_fn)?;
            }
            _ => {}
        }
    }

    out.write_str(&text[last..])
}

fn alt_text<'a>(
    events: &mut Peekable<impl Iterator<Item = (Event<'a>, Range<usize>)>>,
    end: usize,
) -> CowStr<'a> {
    let mut alt = Cow::Borrowed("");
    let mut nested = 0usize;

    while let Some((event, _)) = events.next_if(|(_, range)| range.start < end) {
        match event {
            Event::Start(_) => nested += 1,
            Event::End(_) => {
                if nested == 0 {
                    break;
                }
                nested -= 1;
            }
            Event::Text(text)
            | Event::Code(text)
            | Event::InlineHtml(text)
            | Event::InlineMath(text)
            | Event::DisplayMath(text) => push_alt(&mut alt, text),
            Event::SoftBreak | Event::HardBreak => push_alt(&mut alt, CowStr::Borrowed(" ")),
            _ => {}
        }
    }

    match alt {
        Cow::Borrowed(alt) => CowStr::Borrowed(alt),
        Cow::Owned(alt) => alt.into(),
    }
}

fn push_alt<'a>(alt: &mut Cow<'a, str>, text: CowStr<'a>) {
    if !alt.is_empty() {
        alt.to_mut().push_str(&text);
    } else if let CowStr::Borrowed(text) = text {
        *alt = Cow::Borrowed(text);
    } else {
        *alt = Cow::Owned(text.into_string());
    }
}

fn map_block_images<'a, W, F, R>(
    out: &mut W,
    text: &'a str,
    block: Range<usize>,
    lines: &[Range<usize>],
    map_fn: &mut F,
) -> fmt::Result
where
    W: Write,
    F: FnMut(Image<'a>) -> R,
    R: AsRef<str>,
{
    let source = &text[block.clone()];
    if !contains_img(source) {
        return out.write_str(source);
    }

    let mut starts = Vec::with_capacity(lines.len());
    let mut combined = String::new();
    for line in lines {
        starts.push(combined.len());
        combined.push_str(&text[line.clone()]);
    }
    let map = |pos: usize| {
        let idx = starts.partition_point(|&start| start <= pos) - 1;
        lines[idx].start + (pos - starts[idx])
    };

    let mut last = block.start;
    for tag in img_start_tags(&combined) {
        let span = map(tag.span.start)..map(tag.span.end - 1) + 1;
        out.write_str(&text[last..span.start])?;
        let raw = &text[span.clone()];
        last = span.end;
        let mapped = map_fn(Image::new(span, raw, Kind::html(tag)));
        out.write_str(mapped.as_ref())?;
    }
    out.write_str(&text[last..block.end])
}

#[inline]
fn contains_img(html: &str) -> bool {
    html.as_bytes().windows(4).any(|window| window.eq_ignore_ascii_case(b"<img"))
}

fn img_start_tags(html: &str) -> impl Iterator<Item = StartTag<usize>> {
    let mut emitter = DefaultEmitter::<usize>::new_with_span();
    emitter.naively_switch_states(true);
    Tokenizer::new_with_emitter(html, emitter).flatten().filter_map(|token| match token {
        Token::StartTag(tag) if tag.name == b"img" => Some(tag),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_urls(text: &str, expected: &[Option<&str>]) {
        let mut index = 0;
        map_images_with(
            text,
            Options { markdown: pulldown_cmark::Options::all(), html: true },
            |image| {
                assert_eq!(image.url(), expected[index], "image {index} of {text:?}");
                index += 1;
                image.raw
            },
        );
        assert_eq!(index, expected.len(), "image count of {text:?}");
    }

    #[test]
    fn url_is_empty_or_absent() {
        check_urls("![alt]()", &[Some("")]);
        check_urls("![alt](<>)", &[Some("")]);
        check_urls("![alt][r]\n\n[r]: <>", &[Some("")]);
        check_urls("<div>\n<img>\n</div>", &[None]);
        check_urls("<div>\n<img alt=\"no source\">\n</div>", &[None]);
        check_urls("<div>\n<img src>\n</div>", &[Some("")]);
        check_urls("<div>\n<img src=\"\">\n</div>", &[Some("")]);
        check_urls("<div>\n<img src=\"   \">\n</div>", &[Some("   ")]);
    }

    #[test]
    fn url_is_passed_through_verbatim() {
        check_urls("![alt](a.png)", &[Some("a.png")]);
        check_urls("![alt](./a.png)", &[Some("./a.png")]);
        check_urls("![alt](../a.png)", &[Some("../a.png")]);
        check_urls("![alt](/path/to/train.jpg)", &[Some("/path/to/train.jpg")]);
        check_urls("![alt](a%20b.png)", &[Some("a%20b.png")]);
        check_urls(r"![alt](C:\a\b.png)", &[Some(r"C:\a\b.png")]);
        check_urls("![alt](C|/img.png)", &[Some("C|/img.png")]);
        check_urls("![alt](dir/a:b.png)", &[Some("dir/a:b.png")]);
        check_urls("![alt](we#rd.png)", &[Some("we#rd.png")]);
        check_urls("![alt](a.png?v=2#frag)", &[Some("a.png?v=2#frag")]);
        check_urls("![alt](http://x/a.png)", &[Some("http://x/a.png")]);
        check_urls("![alt](HTTPS://Example.COM/a.png)", &[Some("HTTPS://Example.COM/a.png")]);
        check_urls("![alt](mailto:a@b.c)", &[Some("mailto:a@b.c")]);
        check_urls("![alt](//cdn.example.com/a.png)", &[Some("//cdn.example.com/a.png")]);
        check_urls("![alt](data:image/png;base64,AAAA)", &[Some("data:image/png;base64,AAAA")]);
        check_urls("![alt](data:)", &[Some("data:")]);
    }

    #[test]
    fn url_unwraps_angle_brackets_and_decodes_entities() {
        check_urls("![alt](<my image.png>)", &[Some("my image.png")]);
        check_urls("![alt](&#32;&#32;a.png&#32;)", &[Some("  a.png ")]);
        check_urls("![alt](a.png?x=1&amp;y=2)", &[Some("a.png?x=1&y=2")]);
        check_urls("![alt](&#104;ttps://e.com/a.png)", &[Some("https://e.com/a.png")]);
    }

    #[test]
    fn url_of_wikilinks_is_the_target() {
        check_urls("![[dog.png]]", &[Some("dog.png")]);
        check_urls("![[dog]]", &[Some("dog")]);
        check_urls("![[dog.png|a cute dog]]", &[Some("dog.png")]);
        check_urls("![[ dog.png ]]", &[Some(" dog.png ")]);
        check_urls("![[nested/path/cat.jpg]]", &[Some("nested/path/cat.jpg")]);
        check_urls("![[https://example.com/a.png]]", &[Some("https://example.com/a.png")]);
        check_urls(r"![[image.png\|300]]", &[Some(r"image.png\")]);
        check_urls(r"![[sub\a.png]]", &[Some(r"sub\a.png")]);
        check_urls("![[#heading]]", &[Some("#heading")]);
    }

    #[test]
    fn url_of_html_images() {
        check_urls("<div>\n<img src=one.png/>\n</div>", &[Some("one.png/")]);
        check_urls(
            "<div>\n<img src=\"data:image/png;base64,iVBO\n  Rw0KGgo=\">\n</div>",
            &[Some("data:image/png;base64,iVBO\n  Rw0KGgo=")],
        );
        check_urls(
            "<div>\n<img src=\"data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg'/>\">\n</div>",
            &[Some("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg'/>")],
        );
    }

    #[test]
    fn html_attributes_are_case_insensitive() {
        map_images_with(
            "<div>\n<img SRC=\"a.png\" ALT=\"x\" SrcSet=\"b.png 2x\">\n</div>",
            Options { markdown: pulldown_cmark::Options::all(), html: true },
            |image| {
                let Kind::Html(html) = &image.kind else { panic!("expected html image") };
                assert_eq!(html.attribute("SRC"), Some("a.png"));
                assert_eq!(html.attribute("src"), Some("a.png"));
                assert_eq!(html.attribute("ALT"), Some("x"));
                assert_eq!(html.attribute("SrcSet"), Some("b.png 2x"));
                assert_eq!(html.attribute("nope"), None);
                image.raw
            },
        );
    }
}
