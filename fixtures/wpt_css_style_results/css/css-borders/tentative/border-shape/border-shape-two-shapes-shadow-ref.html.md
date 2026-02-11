# css/css-borders/tentative/border-shape/border-shape-two-shapes-shadow-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-two-shapes-shadow-ref.html"
}
```

## style[0]

```css

    body {
        margin: 0;
    }
    svg {
        position: absolute;
        top: 0;
        left: 0;
        width: 220px;
        height: 220px;
        overflow: visible;
    }
    .outer-shadow {
        fill: blue;
        stroke: blue;
        stroke-width: 10px;
        transform: translate(10px, 10px);
    }
    .outer-border {
        fill: yellow;
    }
    .inner-fill {
        fill: green;
    }
    .inset-shadow {
        fill: none;
        stroke: purple;
        stroke-width: 10px;
    }
```

```json
{
  "errors": 8,
  "messages": [
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
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    },
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
