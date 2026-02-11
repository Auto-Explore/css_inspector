# css/css-backgrounds/animations/background-image-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-image-interpolation.html"
}
```

## style[0]

```css

.parent {
  background-image: url(../resources/blue-100.png);
  background-size: 0 0;
}
.target {
  width: 100px;
  height: 100px;
  display: inline-block;
  border: 10px solid black;
  background-repeat: no-repeat;
  background-image: url(../resources/blue-100.png);
}
.expected {
  border-color: green;
  margin-right: 2px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
