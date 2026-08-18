from pathlib import Path

import pytest
from mdimg import Image, MarkdownOptions, map_images

FIXTURES = sorted((Path(__file__).parents[2] / "mdimg/tests/test_data/fixtures").rglob("*.md"))

OPTIONS = [
    pytest.param(MarkdownOptions.EMPTY, True, id="none"),
    pytest.param(MarkdownOptions.EMPTY, False, id="none_no_html"),
    pytest.param(MarkdownOptions.ALL, True, id="all"),
    pytest.param(MarkdownOptions.ALL, False, id="all_no_html"),
]


def test_fixture_corpus_was_found() -> None:
    assert len(FIXTURES) > 100


@pytest.mark.parametrize(("markdown", "html"), OPTIONS)
@pytest.mark.parametrize("fixture", FIXTURES, ids=lambda p: p.stem)
def test_raw_is_identity(fixture: Path, markdown: int, html: bool) -> None:
    text = fixture.read_bytes().decode("utf-8")
    assert map_images(text, lambda image: image.raw, markdown=markdown, html=html) == text


@pytest.mark.parametrize(("markdown", "html"), OPTIONS)
@pytest.mark.parametrize("fixture", FIXTURES, ids=lambda p: p.stem)
def test_spans_slice_back_to_raw(fixture: Path, markdown: int, html: bool) -> None:
    text = fixture.read_bytes().decode("utf-8")
    images: list[Image] = []
    map_images(text, lambda image: images.append(image) or image.raw, markdown=markdown, html=html)
    for image in images:
        start, end = image.span
        assert text[start:end] == image.raw
