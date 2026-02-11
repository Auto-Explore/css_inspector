# css/css-masking/mask-image/backdrop-filter-bad-mask-image.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/backdrop-filter-bad-mask-image.html"
}
```

## style[0]

```css

  .backdrop-filter {
    backdrop-filter: invert(1);
    background-color: rgba(255, 255, 255, 0.5);
  }

  .bad-mask {
    /* This invalid mask-image should fully-mask this element. */
    mask-image: url("invalid://nonexistent");
  }

  .double-mask {
    /* Multiple invalid mask layers: all invalid, element should be fully masked. */
    mask-image: url("invalid://nonexistent"), url("invalid://nonexistent");
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
