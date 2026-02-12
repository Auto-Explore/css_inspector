# css/css-masking/mask-image/backdrop-filter-good-and-bad-mask-image.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/backdrop-filter-good-and-bad-mask-image.html"
}
```

## style[0]

```css

  body {
    background-color: yellow;
  }

  .double-mask {
    /* Multiple mask layers: invalid mask is ignored, valid mask takes effect. */
    mask-image: url("invalid://nonexistent"), linear-gradient(black, black);
  }

  .backdrop-filter {
    backdrop-filter: invert(1);
    background-color: rgba(255, 255, 255, 0.5);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
