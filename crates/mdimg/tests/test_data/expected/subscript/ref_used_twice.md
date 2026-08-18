Image { span: 0..7, raw: "![a][r]", kind: Markdown(MarkdownImage { url: Borrowed("a.png"), title: Borrowed(""), alt: Borrowed("a"), link: Reference { id: Borrowed("r") } }) } and Image { span: 12..19, raw: "![b][r]", kind: Markdown(MarkdownImage { url: Borrowed("a.png"), title: Borrowed(""), alt: Borrowed("b"), link: Reference { id: Borrowed("r") } }) }

[r]: a.png