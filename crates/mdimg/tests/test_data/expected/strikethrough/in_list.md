- Image { span: 2..15, raw: "![a](one.png)", kind: Markdown(MarkdownImage { url: Borrowed("one.png"), title: Borrowed(""), alt: Borrowed("a"), link: Inline }) }
- nested:
  - Image { span: 30..43, raw: "![b](two.png)", kind: Markdown(MarkdownImage { url: Borrowed("two.png"), title: Borrowed(""), alt: Borrowed("b"), link: Inline }) }

1. Image { span: 48..63, raw: "![c](three.png)", kind: Markdown(MarkdownImage { url: Borrowed("three.png"), title: Borrowed(""), alt: Borrowed("c"), link: Inline }) }