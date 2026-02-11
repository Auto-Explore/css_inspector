# css/fill-stroke/animation/stroke-color-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/fill-stroke/animation/stroke-color-interpolation.html"
}
```

## style[0]

```css

.parent {
  stroke-color: #eee;
}
.target {
  width: 60px;
  height: 60px;
  display: inline-block;
  border: 2px solid black;
  margin-right: 2px;
  color: rgba(0, 0, 255, 0.5);
  stroke-color: black;
}
.expected {
  margin-right: 15px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “stroke-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
