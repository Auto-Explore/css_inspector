# css/css-shapes/animation/shape-outside-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/animation/shape-outside-interpolation.html"
}
```

## style[0]

```css

.parent {
  shape-outside: circle(80% at 30% 10%);
}
.target {
  shape-outside: circle(60% at 10% 30%);
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
