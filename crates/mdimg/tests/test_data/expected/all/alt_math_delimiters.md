Image { span: 0..20, raw: "![$a+b$](inline.png)", kind: Markdown(MarkdownImage { url: Borrowed("inline.png"), title: Borrowed(""), alt: Borrowed("a+b"), link: Inline }) }

Image { span: 22..45, raw: "![$$a+b$$](display.png)", kind: Markdown(MarkdownImage { url: Borrowed("display.png"), title: Borrowed(""), alt: Borrowed("a+b"), link: Inline }) }

Image { span: 47..64, raw: "![a+b](plain.png)", kind: Markdown(MarkdownImage { url: Borrowed("plain.png"), title: Borrowed(""), alt: Borrowed("a+b"), link: Inline }) }

Image { span: 66..93, raw: "![$x$ and $$y$$](mixed.png)", kind: Markdown(MarkdownImage { url: Borrowed("mixed.png"), title: Borrowed(""), alt: Boxed("x and y"), link: Inline }) }