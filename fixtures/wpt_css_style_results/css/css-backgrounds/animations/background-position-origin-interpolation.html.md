# css/css-backgrounds/animations/background-position-origin-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-position-origin-interpolation.html"
}
```

## style[0]

```css

.parent {
  background-position: 80px 80px;
}
.target {
  border: 3px solid skyblue;
  width: 100px;
  height: 100px;
  background-image: linear-gradient(to right, green, green);
  background-size: 20px 20px;
  background-repeat: no-repeat;
  background-position: 10px 10px;
  display: inline-block;
}

.actual {
  background-image: linear-gradient(to right, red, red);
}
.expected {
  margin-left: -106px;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
