# mdimg

This project provides a simple way of surgically substituting image syntax in Markdown. I.e. map image syntax like (`![alt](url)` and `<img src="url" alt="alt">`) to your own lambda function.

The markdown parser is [pulldown-cmark](https://pulldown-cmark.github.io/pulldown-cmark/) and
simply re-exports its functionality and configuration options.

## Example

```python
import mdimg

output = mdimg.map_images(
    "Before ![alt](file.png) after", lambda image: f"[image: alt={image.alt} url={image.url}]"
)
assert output == "Before [image: alt=alt url=file.png] after"
```

Return `image.raw` to leave an image unchanged. Configure markdown parsing with the `markdown` argument, e.g. to enable GitHub Flavored Markdown:

```python
from mdimg import MarkdownOptions, map_images

map_images(text, lambda image: f"your mapping logic here", markdown=MarkdownOptions.GFM)
```
