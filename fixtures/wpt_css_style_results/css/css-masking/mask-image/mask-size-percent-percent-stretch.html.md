# css/css-masking/mask-image/mask-size-percent-percent-stretch.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-size-percent-percent-stretch.html"
}
```

## style[0]

```css

      div {
        width: 60px;
        height: 120px;
      }

      #outer {
        border: 10px solid black;
      }

      #inner {
        background-color: purple;
        mask-image: url(support/transparent-100x50-blue-100x50.svg);
        mask-repeat: no-repeat;
        mask-position: left top;
        mask-size: 100% 100%;
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
