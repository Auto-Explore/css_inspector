# css/css-transforms/transform-box/svgbox-content-box.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-box/svgbox-content-box.html"
}
```

## style[0]

```css

#target {
  fill: green;
  stroke: black;
  stroke-width: 50;
  transform-box: content-box; /* alias for fill-box */
  transform-origin: 0px 0px;
  transform: rotate(90deg);
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
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
