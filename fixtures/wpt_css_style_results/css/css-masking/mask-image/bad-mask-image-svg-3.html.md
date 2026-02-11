# css/css-masking/mask-image/bad-mask-image-svg-3.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/bad-mask-image-svg-3.html"
}
```

## style[0]

```css

  svg > rect {
    /* Single invalid mask layer: Mask is ignored. */
    mask-image: image-set(url("invalid://nonexistent"));
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
