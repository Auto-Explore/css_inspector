# css/CSS2/stacking-context/composite-change-after-scroll-preserves-stacking-order.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/stacking-context/composite-change-after-scroll-preserves-stacking-order.html"
}
```

## style[0]

```css

    #one, #two {
        width: 200px; height: 200px; background: lightblue; position: relative
    }
    #one {
        background: lightblue
    }
    #two {
        background: lightgray;
        margin-top: -200px;
    }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
