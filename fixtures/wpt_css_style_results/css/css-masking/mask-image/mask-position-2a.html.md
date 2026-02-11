# css/css-masking/mask-image/mask-position-2a.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-position-2a.html"
}
```

## style[0]

```css

      div {
        width: 100px;
        height: 100px;
      }

      #outer {
        border: 1px solid black;
      }

      #inner {
        background-color: purple;
        mask-position: left 40% bottom 60%;
        mask-image: url(support/50x50-opaque-blue.svg);
        mask-repeat: no-repeat;
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “mask-position”.",
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
