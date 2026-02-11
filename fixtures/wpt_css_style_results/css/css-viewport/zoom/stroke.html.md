# css/css-viewport/zoom/stroke.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/stroke.html"
}
```

## style[0]

```css

html, body {
  margin: 0;
}
svg {
  display: block;
}
.dash {
  stroke-width: 6px;
  stroke-dasharray: 10px;
  stroke-dashoffset: 7px;
  stroke: hotpink;
  fill: none;
}
.zoom {
  zoom: 2;
}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-dasharray”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-dashoffset”.",
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
