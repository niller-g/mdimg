Image { span: 0..17, raw: "![a \"title 1\"][r]", kind: Markdown(MarkdownImage { url: Borrowed("a.png"), title: Borrowed("title 2"), alt: Borrowed("a \"title 1\""), link: Reference { id: Borrowed("r") } }) }

[r]: a.png "title 2"