# css/css-transforms/transform-3d-scales-different-x-y-dynamic-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-3d-scales-different-x-y-dynamic-002-ref.html"
}
```

## style[0]

```css

body {
  margin: 0;
  overflow: clip;
}
.test:nth-of-type(even) {
  color: white;
}
.test {
  position: absolute;
  width: 0;
  transform-origin: 0 0;
  border-right: 25px solid transparent;
  border-top: calc(25px * var(--scale)) solid currentcolor;
  transform: scale3d(var(--scale), 1, 1);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
