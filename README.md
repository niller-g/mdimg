# mdimg

> [!WARNING]  
> This project is still work-in-progress. Please submit issues and PRs if you want to help out.

This project provides a simple way of surgically substituting image syntax in Markdown. I.e. map image syntax like (`![alt](url)` and `<img src="url" alt="alt">`) to your own lambda function.

The markdown parser is [pulldown-cmark](https://pulldown-cmark.github.io/pulldown-cmark/) and simply re-exports its functionality and configuration options.

This is useful in many different scenarios, for example when you want an inlined-only markdown document, or when you want to replace images with their AI-described textual description.

## Contents

- [`crates/mdimg`](crates/mdimg) — the core library crate.
- [`crates/mdimg-python`](crates/mdimg-python) — a Python wrapper around the core library crate.
