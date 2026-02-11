# css/css-masking/mask-image/mask-image-3i.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-3i.html"
}
```

## style[0]

```css

      rect {
        fill: purple;
        mask-image: url(#mask1), url(support/50x50-opaque-blue.svg);
        mask-repeat: no-repeat, no-repeat;
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-repeat”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
