from enum import IntFlag

from ._mdimg import (  # pyright: ignore[reportMissingModuleSource]
    HtmlImage,
    Image,
    Link,
    MarkdownImage,
    map_images,
)


class MarkdownOptions(IntFlag):
    """Markdown extensions to enable, passed as the `markdown` argument of `map_images`.

    Flags combine with `|`; `EMPTY` enables none and `ALL` enables every extension. They mirror
    the extension options of pulldown-cmark.
    """

    EMPTY = 0
    TABLES = 1 << 1
    FOOTNOTES = 1 << 2
    STRIKETHROUGH = 1 << 3
    TASKLISTS = 1 << 4
    SMART_PUNCTUATION = 1 << 5
    HEADING_ATTRIBUTES = 1 << 6
    YAML_STYLE_METADATA_BLOCKS = 1 << 7
    PLUSES_DELIMITED_METADATA_BLOCKS = 1 << 8
    OLD_FOOTNOTES = (1 << 9) | (1 << 2)
    MATH = 1 << 10
    GFM = 1 << 11
    DEFINITION_LIST = 1 << 12
    SUPERSCRIPT = 1 << 13
    SUBSCRIPT = 1 << 14
    WIKILINKS = 1 << 15
    ALL = (
        TABLES
        | FOOTNOTES
        | STRIKETHROUGH
        | TASKLISTS
        | SMART_PUNCTUATION
        | HEADING_ATTRIBUTES
        | YAML_STYLE_METADATA_BLOCKS
        | PLUSES_DELIMITED_METADATA_BLOCKS
        | OLD_FOOTNOTES
        | MATH
        | GFM
        | DEFINITION_LIST
        | SUPERSCRIPT
        | SUBSCRIPT
        | WIKILINKS
    )


__all__ = ["HtmlImage", "Image", "Link", "MarkdownImage", "MarkdownOptions", "map_images"]
