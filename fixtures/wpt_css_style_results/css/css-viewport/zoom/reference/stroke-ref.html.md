# css/css-viewport/zoom/reference/stroke-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/reference/stroke-ref.html"
}
```

## style[0]

```css

html, body {
  margin: 0;
  --scale: 1;
}
svg {
  display: block;
}
.zoom {
  --scale: 2;
}
.dash {
  stroke-width: calc(6px * var(--scale));
  stroke-dasharray: calc(10px * var(--scale));
  stroke-dashoffset: calc(7px * var(--scale));
  stroke: hotpink;
  fill: none;
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
