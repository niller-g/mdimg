<div>
![a](one.png)
</div>

Image { span: 28..47, raw: "<img src=\"two.png\">", kind: Html({"src": "two.png"}) } and Image { span: 52..91, raw: "<img src=\"data:image/png;base64,AAAA=\">", kind: Html({"src": "data:image/png;base64,AAAA="}) }

Inline <span>Image { span: 106..121, raw: "![b](three.png)", kind: Markdown(MarkdownImage { url: Borrowed("three.png"), title: Borrowed(""), alt: Borrowed("b"), link: Inline }) }</span> html.