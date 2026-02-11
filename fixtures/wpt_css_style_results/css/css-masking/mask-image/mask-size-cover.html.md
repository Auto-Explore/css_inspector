# css/css-masking/mask-image/mask-size-cover.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-size-cover.html"
}
```

## style[0]

```css

      #outer {
        border: 10px solid black;
        width: 64px;
        height: 128px;
        background-color: purple;
        mask-image: url(support/50x50-opaque-blue.svg);
        mask-repeat: no-repeat;
        mask-position: left top;
        mask-origin: content-box;
        mask-clip: content-box;
        mask-size: cover;
      }
    
```

```json
{
  "errors": 6,
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
    },
    {
      "message": "Unknown property “mask-origin”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-clip”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
