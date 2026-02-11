# css/css-transforms/transform-box/stroke-box-mutation-004.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-box/stroke-box-mutation-004.html"
}
```

## style[0]

```css

#target {
  transform-box: stroke-box;
  transform-origin: 20px 0px;
  transform: rotate(90deg);
}
#target > rect {
  fill: green;
  stroke: black;
  stroke-width: 20;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
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
