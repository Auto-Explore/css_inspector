# css/css-masking/mask-image/mask-repeat-2.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-repeat-2.html"
}
```

## style[0]

```css

      div {
        width: 128px;
        height: 128px;
      }

      div.outer {
        border: 1px solid black;
      }

      div.inner {
        background-color: purple;
        mask-image: url(support/50x50-opaque-blue.svg);
        mask-position: left top;
      }

      #space {
        mask-repeat: space;
      }
      #space-x {
        mask-repeat: space no-repeat;
      }
      #space-y {
        mask-repeat: no-repeat space;
      }
    
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-position”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-repeat”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-repeat”.",
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
