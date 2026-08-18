# mdimg

> [!WARNING]  
> This project is still work-in-progress. Please submit issues and PRs if you want to help out.

This project provides a simple way of surgically substituting image syntax in Markdown. I.e. map image syntax like (`![alt](url)` and `<img src="url" alt="alt">`) to your own lambda function.

The markdown parser is [pulldown-cmark](https://pulldown-cmark.github.io/pulldown-cmark/) and simply re-exports its functionality and configuration options.

This is useful in many different scenarios, for example when you want an inlined-only markdown document, or when you want to replace images with their AI-described textual description.

## Contents

- [`crates/mdimg`](crates/mdimg) — the core library crate.
- [`crates/mdimg-python`](crates/mdimg-python) — Python (>=3.10) bindings to the core library crate.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for information on how to contribute to this project.

## Development

### Rust

Normal Cargo development workflow applies.

Import checks:

```bash
cargo test
```

Baseline snapshots by setting environment variable `UPDATE_EXPECT=1` and run `cargo test` again.

Everything is checked strictly with Clippy.

```bash
cargo clippy
```

Check performance with benchmarks:

```bash
cargo bench
```

### Python

In `crates/mdimg-python`, create a virtual environment and install dependencies:

```bash
python -m venv .venv
pip install . --group dev
```

Everything is checked strictly with Ruff and Pyright. See [`pyproject.toml`](crates/mdimg-python/pyproject.toml) and [`pyrightconfig.json`](pyrightconfig.json) for configuration.
