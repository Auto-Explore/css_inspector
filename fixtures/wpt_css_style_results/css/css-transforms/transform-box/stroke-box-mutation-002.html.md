# css/css-transforms/transform-box/stroke-box-mutation-002.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-box/stroke-box-mutation-002.html"
}
```

## style[0]

```css

#target {
  fill: green;
  stroke: black;
  stroke-width: 20;
  transform-box: stroke-box;
  transform-origin: 20px 0px;
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
