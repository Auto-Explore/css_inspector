# css/css-masking/mask-image/mask-size-contain-clip-padding.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-size-contain-clip-padding.html"
}
```

## style[0]

```css

      #outer {
        border: 1px solid black;
        width: 64px;
        height: 128px;
      }

      #inner {
        background-color: purple;
        border: 20px solid transparent;
        width: 24px;
        height: 88px;
        mask-image: url(support/50x50-opaque-blue.svg);
        mask-repeat: no-repeat;
        mask-position: left top;
        mask-clip: padding-box;
        mask-origin: padding-box;
        mask-size: contain;
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
      "message": "Unknown property “mask-clip”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-origin”.",
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
