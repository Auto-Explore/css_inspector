# css/css-transforms/preserve3d-and-flattening-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/preserve3d-and-flattening-001-ref.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

.reference {
  background: blue;
  width: 100px;
  height: 100px;
  transform-origin: 0 0;
  transform: translate(50px, 100px) perspective(1000px) rotateX(30deg) translateY(50px) translateZ(100px);
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
