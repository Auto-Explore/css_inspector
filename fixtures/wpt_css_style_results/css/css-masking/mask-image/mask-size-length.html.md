# css/css-masking/mask-image/mask-size-length.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-size-length.html"
}
```

## style[0]

```css

      div {
        width: 64px;
        height: 128px;
      }

      #outer {
        border: 1px solid black;
      }

      #inner {
        background-color: purple;
        mask-image: url(support/50x100-opaque-blue.svg);
        mask-repeat: no-repeat;
        mask-position: left top;
        mask-size: 32px;
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
