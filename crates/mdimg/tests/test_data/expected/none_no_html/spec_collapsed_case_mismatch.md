Image { span: 0..8, raw: "![Foo][]", kind: Markdown(MarkdownImage { url: Borrowed("/url"), title: Borrowed("title"), alt: Borrowed("Foo"), link: Collapsed { id: Borrowed("Foo") } }) }

[foo]: /url "title"