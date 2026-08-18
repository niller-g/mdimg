Image { span: 0..9, raw: "![alt][r]", kind: Markdown(MarkdownImage { url: Borrowed("a.png"), title: Borrowed("the title"), alt: Borrowed("alt"), link: Reference { id: Borrowed("r") } }) }

[r]:
   a.png
   "the title"