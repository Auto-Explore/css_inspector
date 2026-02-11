# css/css-viewport/zoom/svg.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/svg.html"
}
```

## style[0]

```css

  :root {
    font-size: 10px;
    zoom: 2;
  }
  body { margin: 0 }
  .container {
    font-size: 20px;
  }
  .child {
    zoom: 2;
  }
  line {
    stroke-width: 10px;
    stroke: lime;
  }
  polygon, polyline, text {
    fill: lime;
  }
  text {
    font: 10px/1 Ahem;
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
