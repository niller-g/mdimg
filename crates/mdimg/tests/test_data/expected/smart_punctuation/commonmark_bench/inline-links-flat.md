Valid links:

 [this is a link]()
 [this is a link](<http://something.example.com/foo/bar>)
 [this is a link](http://something.example.com/foo/bar 'test')
 Image { span: 156..177, raw: "![this is an image]()", kind: Markdown(MarkdownImage { url: Borrowed(""), title: Borrowed(""), alt: Borrowed("this is an image"), link: Inline }) }
 Image { span: 179..238, raw: "![this is an image](<http://something.example.com/foo/bar>)", kind: Markdown(MarkdownImage { url: Borrowed("http://something.example.com/foo/bar"), title: Borrowed(""), alt: Borrowed("this is an image"), link: Inline }) }
 Image { span: 240..304, raw: "![this is an image](http://something.example.com/foo/bar 'test')", kind: Markdown(MarkdownImage { url: Borrowed("http://something.example.com/foo/bar"), title: Boxed("test"), alt: Borrowed("this is an image"), link: Inline }) }
 
 [escape test](<\>\>\>\>\>\>\>\>\>\>\>\>\>\>> '\'\'\'\'\'\'\'\'\'\'\'\'\'\'')
 [escape test \]\]\]\]\]\]\]\]\]\]\]\]\]\]\]\]](\)\)\)\)\)\)\)\)\)\)\)\)\)\))

Invalid links:

 [this is not a link

 [this is not a link](

 [this is not a link](http://something.example.com/foo/bar 'test'
 
 [this is not a link](((((((((((((((((((((((((((((((((((((((((((((((
 
 [this is not a link]((((((((((()))))))))) (((((((((()))))))))))
