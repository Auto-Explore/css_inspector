# css/css-viewport/zoom/svg-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/svg-ref.html"
}
```

## style[0]

```css

  body { margin: 0 }
  :root {
    font-size: 10px;
  }
  .container {
    font-size: 20px;
  }
  line {
    stroke-width: 40px;
    stroke: lime;
  }
  polygon, polyline, text {
    fill: lime;
  }
  text {
    font: 40px/1 Ahem;
  }
  svg {
    background-color: red;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
