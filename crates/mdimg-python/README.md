# mdimg

This project provides a simple way of surgically substituting image syntax in Markdown. I.e. map image syntax like (`![alt](url)` and `<img src="url" alt="alt">`) to your own lambda function.

```console
pip install mdimg
```

```python
import mdimg

output = mdimg.map_images(
    "Before ![alt](file.png) after", lambda image: f"[image: alt={image.alt} url={image.url}]"
)
assert output == "Before [image: alt=alt url=file.png] after"
```

Return `image.raw` to leave an image unchanged. Markdown parsing is done by
[pulldown-cmark](https://pulldown-cmark.github.io/pulldown-cmark/); its extensions are available
through the `markdown` keyword argument, and HTML `<img>` handling through `html`.

```python
from mdimg import MarkdownOptions, map_images

map_images(text, rewrite, markdown=MarkdownOptions.GFM | MarkdownOptions.WIKILINKS, html=False)
```

## Development

In `crates/mdimg-python`, create a virtual environment and install dependencies:

```bash
python -m venv .venv
pip install . --group dev
```
