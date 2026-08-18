# mdimg

This project provides a simple way of surgically substituting image syntax in Markdown. I.e. map image syntax like (`![alt](url)` and `<img src="url" alt="alt">`) to your own lambda function.

The markdown parser is [pulldown-cmark](https://pulldown-cmark.github.io/pulldown-cmark/) and
simply re-exports its functionality and configuration options.

## Example

```rust
let output = mdimg::map_images("Before ![alt](file.png) after", |image| {
    format!("[image: alt={} url={}]", image.alt().unwrap_or(""), image.url().unwrap())
});
assert_eq!(output, "Before [image: alt=alt url=file.png] after");
```

Return `image.raw` to leave an image unchanged. Configure markdown parsing with the `map_images_with` function, e.g. to enable GitHub Flavored Markdown:

```rust
use mdimg::{map_images_with, Options};

map_images_with(text, 
    Options {
        markdown: mdimg::pulldown_cmark::Options::ENABLE_GFM,
        ..Options::default()
    },
    |image| {
        todo!("Your image mapping logic here")
    }
);
```
