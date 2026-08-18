Image { span: 0..14, raw: "![*foo* bar][]", kind: Markdown(MarkdownImage { url: Borrowed("/url"), title: Borrowed("title"), alt: Boxed("foo bar"), link: Collapsed { id: Borrowed("*foo* bar") } }) }

[*foo* bar]: /url "title"