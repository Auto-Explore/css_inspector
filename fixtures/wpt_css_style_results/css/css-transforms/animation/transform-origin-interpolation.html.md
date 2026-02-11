# css/css-transforms/animation/transform-origin-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/transform-origin-interpolation.html"
}
```

## style[0]

```css

.parent {
  transform-origin: 30px 10px;
}
.target {
  display: inline-block;
  margin-top: 50px;
  margin-bottom: 25px;
  width: 50px;
  height: 50px;
  background: red;
  transform: scale(1.5);
  transform-origin: 10px 30px;
}
.expected {
  background: green;
  position: relative;
  left: -50px;
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
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
