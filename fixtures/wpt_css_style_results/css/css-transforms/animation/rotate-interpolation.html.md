# css/css-transforms/animation/rotate-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/rotate-interpolation.html"
}
```

## style[0]

```css

.parent {
  rotate: 90deg;
}

.target {
  width: 40px;
  height: 20px;
  background-color: grey;
  rotate: 10deg;
}

.expected {
  background-color: green;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
