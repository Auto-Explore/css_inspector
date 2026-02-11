# css/css-viewport/zoom/svg-font-relative-units.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/svg-font-relative-units.html"
}
```

## style[0]

```css

  :root {
    font: 10px/1 Ahem;
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
    stroke-width: 2px;
    stroke: lime;
  }
  svg {
    background-color: black;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
