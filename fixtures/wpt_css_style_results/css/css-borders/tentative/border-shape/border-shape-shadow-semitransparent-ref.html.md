# css/css-borders/tentative/border-shape/border-shape-shadow-semitransparent-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-shadow-semitransparent-ref.html"
}
```

## style[0]

```css

    body {
        margin: 0;
        background: white;
    }
    svg {
        position: absolute;
        top: 0;
        left: 0;
        width: 230px;
        height: 240px;
        overflow: visible;
    }
    .shadow {
        fill: rgba(0, 0, 255, 0.5);
        transform: translate(10px, 20px);
    }
    .border {
        fill: green;
        stroke: black;
        stroke-width: 10px;
    }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
