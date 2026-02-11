# css/css-viewport/zoom/reference/text-stroke-width-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/reference/text-stroke-width-ref.html"
}
```

## style[0]

```css

body {
  --scale: 1;
}
div {
  font-size: calc(3rem * var(--scale));
  -webkit-text-stroke-width: calc(2px * var(--scale));
  color: cornflowerblue;
  -webkit-text-stroke-color: hotpink;
}
.zoom {
  --scale: 2;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “-webkit-text-stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-text-stroke-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
