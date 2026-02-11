# css/css-masking/mask-image/mask-position-7.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-position-7.html"
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
        mask-position: right 0% top 40%;
      }

      #inner2 {
        mask-position: right 0px top 20px;
      }

      #inner3 {
        mask-position: right 0% bottom 60%;
      }

      #inner4 {
        mask-position: right 0px bottom 30px;
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
