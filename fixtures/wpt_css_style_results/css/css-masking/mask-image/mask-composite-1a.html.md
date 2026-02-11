# css/css-masking/mask-image/mask-composite-1a.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-composite-1a.html"
}
```

## style[0]

```css

      div {
        background-color: blue;
        position: absolute;
        margin: 0px;
        padding: 0px;
        width: 100px;
        height: 100px;
        top:10px;
        mask-image: url(support/blue-100x50-transparent-100x50.svg),
                    url(support/blue-100x50-transparent-100x50.svg);
      }

      div.add {
        left: 10px;
        mask-composite: add;
      }

      div.subtract {
        left: 120px;
        mask-composite: subtract;
      }

      div.intersect {
        left: 230px;
        mask-composite: intersect;
      }

      div.exclude {
        left: 340px;
        mask-composite: exclude;
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
      "message": "Unknown property “mask-composite”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-composite”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-composite”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-composite”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
