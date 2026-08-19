from collections.abc import Callable
from typing import final

class Link:
    """The type of link used in a markdown image."""

    @final
    class Inline(Link):
        """An inline link like `[foo](bar)`, an autolink like `<http://foo.bar/baz>`, or an
        email address in an autolink like `<john@example.org>`."""

        __match_args__ = ()

        def __init__(self) -> None: ...

    @final
    class Reference(Link):
        """A reference link like `[foo][bar]` where `bar` is the `id`."""

        __match_args__ = ("id",)

        def __init__(self, id: str) -> None: ...  # noqa: A002
        @property
        def id(self) -> str:
            """The reference label."""

    @final
    class Collapsed(Link):
        """A collapsed link like `[foo][]` where `foo` is the `id`."""

        __match_args__ = ("id",)

        def __init__(self, id: str) -> None: ...  # noqa: A002
        @property
        def id(self) -> str:
            """The reference label."""

    @final
    class Shortcut(Link):
        """A shortcut link like `[foo]` where `foo` is the `id`."""

        __match_args__ = ("id",)

        def __init__(self, id: str) -> None: ...  # noqa: A002
        @property
        def id(self) -> str:
            """The reference label."""

    @final
    class Wiki(Link):
        """A wikilink like `[[foo]]` or `[[foo|bar]]`."""

        __match_args__ = ("has_pothole",)

        def __init__(self, has_pothole: bool) -> None: ...
        @property
        def has_pothole(self) -> bool:
            """`True` if the wikilink was piped: `[[foo|bar]]` is `True`, `[[foo]]` is `False`."""

@final
class MarkdownImage:
    """A markdown image such as `![alt](url "title")`."""

    @property
    def url(self) -> str:
        """The destination URL (references are followed)."""

    @property
    def title(self) -> str:
        """The title of the image. This is the optional text in quotes after the URL, e.g.
        `![alt](url "title")`. References are followed, so it will be the title on the destination
        URL."""

    @property
    def alt(self) -> str:
        """The alternative text of the image, with any inline markup flattened to text."""

    @property
    def link(self) -> Link.Inline | Link.Reference | Link.Collapsed | Link.Shortcut | Link.Wiki:
        """The kind of link the image was written as."""

@final
class HtmlImage:
    """An HTML image such as `<img src="url" alt="alt">`."""

    @property
    def attributes(self) -> dict[str, str]:
        """The attributes of the tag, keyed by name exactly as written in the document."""

    @property
    def self_closing(self) -> bool:
        """`True` if the tag is self-closing, `False` otherwise."""

    def attribute(self, name: str) -> str | None:
        """Returns the value of the given attribute, or `None` if it does not exist. `name` is
        case-insensitive."""

@final
class Image:
    """A markdown or HTML image."""

    @property
    def span(self) -> tuple[int, int]:
        """The `(start, end)` character offsets of the image in the original text."""

    @property
    def raw(self) -> str:
        """The raw text of the image. Return this from your `map_fn` to leave the image
        unchanged."""

    @property
    def url(self) -> str | None:
        """Returns the URL of the image.

        For markdown this is the destination URL (references are followed).
        For HTML this is the value of the `src` attribute (if it exists)."""

    @property
    def alt(self) -> str | None:
        """The alternative text of the image, or `None` if it has none."""

    @property
    def kind(self) -> MarkdownImage | HtmlImage:
        """The kind of image, either a `MarkdownImage` or an `HtmlImage`."""

def map_images(
    text: str, map_fn: Callable[[Image], str], *, markdown: int = 0, html: bool = True
) -> str:
    """Map all images in the given markdown `text` using the provided `map_fn`.

    Every image is passed to `map_fn` as an `Image` and replaced by the string it returns; every
    other byte of the document is left untouched. Return `image.raw` to leave an image unchanged.

    `markdown` selects the pulldown-cmark extensions to enable, and is normally a combination of
    `MarkdownOptions` flags. `html` controls whether HTML `<img>` tags are mapped as well.

    Exceptions raised by `map_fn` propagate out of this function.

    ```python
    output = mdimg.map_images(
        "Before ![alt](file.png) after", lambda image: f"[image: alt={image.alt} url={image.url}]"
    )
    assert output == "Before [image: alt=alt url=file.png] after"
    ```
    """

def markdown_flags() -> dict[str, int]: ...
