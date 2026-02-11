# css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-integer-overflow-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-integer-overflow-crash.html"
}
```

## style[0]

```css

.a {
  float: left;
  min-height: 99vw;
  shape-outside: ellipse(61% 100% at 34% 62%);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
