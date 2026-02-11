# css/css-masking/mask-image/mask-position-6.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-position-6.html"
}
```

## style[0]

```css

      div {
        width: 100px;
        height: 100px;
      }

      .outer {
        border: 1px solid black;
      }

      .outer > div {
        background-color: purple;
        mask-image: url(support/50x50-opaque-blue.svg);
        mask-repeat: no-repeat;
      }

      #inner1 {
        mask-position: left 20px bottom 0px;
      }

      #inner2 {
        mask-position: left 40% bottom 0%;
      }

      #inner3 {
        mask-position: right 60% bottom 0%;
      }

      #inner4 {
        mask-position: right 30px bottom 0px;
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
      "message": "Unknown property “mask-position”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-position”.",
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
