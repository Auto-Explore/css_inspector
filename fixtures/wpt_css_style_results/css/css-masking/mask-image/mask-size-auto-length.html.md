# css/css-masking/mask-image/mask-size-auto-length.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-size-auto-length.html"
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
        width: 64px;
        height: 128px;
        background-color: purple;
        mask-image: url(support/50x50-opaque-blue.svg);
        mask-repeat: no-repeat;
        mask-position: left top;
        mask-size: auto 20px;
      }
    
```

```json
{
  "errors": 4,
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
      "message": "Unknown property “mask-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
