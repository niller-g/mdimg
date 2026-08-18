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
