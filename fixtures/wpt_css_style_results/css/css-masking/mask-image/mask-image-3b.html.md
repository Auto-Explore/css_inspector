# css/css-masking/mask-image/mask-image-3b.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-3b.html"
}
```

## style[0]

```css

      div {
        background-color: purple;
        mask-image: url(support/50x50-opaque-blue.svg), url(#mask1);
        mask-repeat: no-repeat, repeat;
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
      "message": "Unknown property “mask-image”.",
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
