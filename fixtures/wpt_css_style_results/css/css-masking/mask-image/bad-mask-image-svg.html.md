# css/css-masking/mask-image/bad-mask-image-svg.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/bad-mask-image-svg.html"
}
```

## style[0]

```css

  svg > rect {
    /* Multiple invalid mask layers: all invalid, SVG element should be fully masked. */
    mask-image: url("invalid://nonexistent"), url("invalid://nonexistent");
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
