# css/css-masking/mask-image/mask-size-contain-position-fifty-fifty.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-size-contain-position-fifty-fifty.html"
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
        mask-image: url(support/50x50-opaque-blue.svg);
        mask-repeat: no-repeat;
        mask-position: left top;
        mask-size: contain;
        mask-position: 50% 50%;
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
