# css/css-backgrounds/animations/border-color-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/border-color-interpolation.html"
}
```

## style[0]

```css

.parent {
  border-color: white;
}
.target {
  width: 50px;
  height: 50px;
  background-color: blue;
  display: inline-block;
  border: 12px solid;
  border-color: darkblue;
}
.expected {
  background-color: green;
  margin-right: 2px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
