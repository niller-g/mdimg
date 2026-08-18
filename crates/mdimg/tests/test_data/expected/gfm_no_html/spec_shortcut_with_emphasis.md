Image { span: 0..12, raw: "![foo *bar*]", kind: Markdown(MarkdownImage { url: Borrowed("train.jpg"), title: Borrowed("train & tracks"), alt: Boxed("foo bar"), link: Shortcut { id: Borrowed("foo *bar*") } }) }

[foo *bar*]: train.jpg "train & tracks"