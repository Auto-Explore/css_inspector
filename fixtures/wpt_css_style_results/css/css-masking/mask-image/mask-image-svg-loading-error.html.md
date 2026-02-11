# css/css-masking/mask-image/mask-image-svg-loading-error.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-svg-loading-error.html"
}
```

## style[0]

```css

  svg > rect {
    mask-image: image-set(url("support/green-100x100.png?pipe=slice(1,200)"));
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
