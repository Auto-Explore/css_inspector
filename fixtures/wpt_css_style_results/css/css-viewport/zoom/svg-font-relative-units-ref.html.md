# css/css-viewport/zoom/svg-font-relative-units-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/svg-font-relative-units-ref.html"
}
```

## style[0]

```css

  :root {
    font: 10px/1 Ahem;
  }
  body { margin: 0 }
  .container {
    font-size: 20px;
  }
  line {
    stroke-width: 8px;
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
