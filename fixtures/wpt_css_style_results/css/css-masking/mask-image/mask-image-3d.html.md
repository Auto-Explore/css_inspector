# css/css-masking/mask-image/mask-image-3d.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-3d.html"
}
```

## style[0]

```css

      div {
        background-color: purple;
        mask: url(#mask1), url(support/50x50-opaque-blue.svg);
        mask-repeat: no-repeat, no-repeat;
        mask-position: 0 0, 0 0;
        width: 100px;
        height: 100px;
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “mask”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-repeat”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
