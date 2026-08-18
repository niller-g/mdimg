import pytest
from mdimg import HtmlImage, Image, Link, MarkdownImage, MarkdownOptions, map_images
from mdimg._mdimg import markdown_flags


def test_maps_a_markdown_image() -> None:
    out = map_images(
        "Before ![alt](file.png) after", lambda image: f"[image: alt={image.alt} url={image.url}]"
    )
    assert out == "Before [image: alt=alt url=file.png] after"


def test_returning_raw_leaves_the_document_unchanged() -> None:
    text = '![a](x.png)\n\n<img src="y.png">\n\ntail\n'
    assert map_images(text, lambda image: image.raw) == text


def test_text_without_images_is_untouched() -> None:
    text = "no images here\n"
    assert map_images(text, lambda _: "X") == text


def test_markdown_image_fields() -> None:
    kind = collect('![alt](url.png "the title")')[0].kind
    assert isinstance(kind, MarkdownImage)
    assert (kind.url, kind.title, kind.alt) == ("url.png", "the title", "alt")
    assert isinstance(kind.link, Link.Inline)


@pytest.mark.parametrize(
    ("text", "variant", "attribute", "value"),
    [
        ("![a](b.png)", Link.Inline, None, None),
        ("![a][ref]\n\n[ref]: b.png\n", Link.Reference, "id", "ref"),
        ("![a][]\n\n[a]: b.png\n", Link.Collapsed, "id", "a"),
        ("![a]\n\n[a]: b.png\n", Link.Shortcut, "id", "a"),
    ],
)
def test_link_variants(
    text: str, variant: type[Link], attribute: str | None, value: str | None
) -> None:
    kind = collect(text)[0].kind
    assert isinstance(kind, MarkdownImage)
    assert isinstance(kind.link, variant)
    if attribute is not None:
        assert getattr(kind.link, attribute) == value


@pytest.mark.parametrize(
    ("text", "has_pothole"), [("![[target]]", False), ("![[target|label]]", True)]
)
def test_wikilink_variants(text: str, has_pothole: bool) -> None:
    kind = collect(text, markdown=MarkdownOptions.WIKILINKS)[0].kind
    assert isinstance(kind, MarkdownImage)
    assert isinstance(kind.link, Link.Wiki)
    assert kind.link.has_pothole is has_pothole


def test_link_variants_support_pattern_matching() -> None:
    kind = collect("![a][ref]\n\n[ref]: b.png\n")[0].kind
    assert isinstance(kind, MarkdownImage)
    match kind.link:
        case Link.Reference(id=found):
            assert found == "ref"
        case other:
            pytest.fail(f"expected Link.Reference, got {other!r}")


def test_html_image_fields() -> None:
    kind = collect('<img SRC="y.png" ALT="z" width=10 />')[0].kind
    assert isinstance(kind, HtmlImage)
    assert kind.attributes == {"src": "y.png", "alt": "z", "width": "10"}
    assert kind.self_closing is True


def test_html_attribute_lookup_is_case_insensitive() -> None:
    kind = collect('<img src="a.png">')[0].kind
    assert isinstance(kind, HtmlImage)
    assert kind.attribute("SRC") == kind.attribute("src") == "a.png"
    assert kind.attribute("nope") is None


def test_html_image_without_src_has_no_url() -> None:
    image = collect("<img alt=no-source>")[0]
    assert image.url is None
    assert image.alt == "no-source"


def test_html_can_be_disabled() -> None:
    text = '<img src="a.png">'
    assert map_images(text, lambda _: "X", html=False) == text


def test_markdown_extensions_are_off_by_default() -> None:
    text = "![[target]]"
    assert map_images(text, lambda _: "X") == text
    assert map_images(text, lambda _: "X", markdown=MarkdownOptions.WIKILINKS) == "X"


def test_markdown_options_combine() -> None:
    options = MarkdownOptions.GFM | MarkdownOptions.WIKILINKS
    assert map_images("![[target]]", lambda _: "X", markdown=options) == "X"


def test_unknown_option_bits_are_ignored() -> None:
    assert map_images("![a](b.png)", lambda _: "X", markdown=1 << 31) == "X"


def test_negative_option_bits_enable_every_extension() -> None:
    assert map_images("![[target]]", lambda _: "X", markdown=-1) == "X"


def test_inverted_options_enable_every_extension() -> None:
    assert map_images("![[target]]", lambda _: "X", markdown=~MarkdownOptions(0)) == "X"


def test_options_match_pulldown_cmark() -> None:
    flags = {name.removeprefix("ENABLE_"): bits for name, bits in markdown_flags().items()}
    members = {
        name: int(member)
        for name, member in MarkdownOptions.__members__.items()
        if name not in ("EMPTY", "ALL")
    }
    assert members == flags

    combined = MarkdownOptions.EMPTY
    for bits in flags.values():
        combined |= MarkdownOptions(bits)
    assert MarkdownOptions.ALL == combined


def test_span_is_in_character_offsets() -> None:
    text = "æøå 🎉 ![alt](p.png) tail"
    image = collect(text)[0]
    start, end = image.span
    assert text[start:end] == image.raw == "![alt](p.png)"


def test_spans_are_correct_for_several_images() -> None:
    text = "🎉 ![a](1.png) æø ![b](2.png) å"
    for image in collect(text):
        start, end = image.span
        assert text[start:end] == image.raw


def test_callback_exception_propagates() -> None:
    def boom(_image: Image) -> str:
        raise ValueError("callback exploded")

    with pytest.raises(ValueError, match="callback exploded"):
        map_images("![a](b.png)", boom)


def test_callback_exception_stops_further_calls() -> None:
    calls: list[str] = []

    def boom(image: Image) -> str:
        calls.append(image.raw)
        raise KeyError("nope")

    with pytest.raises(KeyError):
        map_images("![a](1.png) ![b](2.png) ![c](3.png)", boom)
    assert calls == ["![a](1.png)"]


def test_non_string_return_raises_type_error() -> None:
    with pytest.raises(TypeError):
        map_images("![a](b.png)", lambda _: 42)  # pyright: ignore[reportArgumentType]


def test_repr_shows_the_image_contents() -> None:
    image = collect("![a](b.png)")[0]
    assert repr(image) == (
        'Image(span=(0, 11), raw="![a](b.png)", '
        'kind=MarkdownImage(url="b.png", title="", alt="a", link=Link.Inline()))'
    )


def test_images_are_the_documented_types() -> None:
    image = collect("![a](b.png)")[0]
    assert isinstance(image, Image)


def collect(text: str, *, markdown: int = 0, html: bool = True) -> list[Image]:
    images: list[Image] = []
    map_images(text, lambda image: images.append(image) or image.raw, markdown=markdown, html=html)
    return images
