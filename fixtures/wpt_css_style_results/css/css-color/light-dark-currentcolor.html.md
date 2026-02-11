# css/css-color/light-dark-currentcolor.html

```json
{
  "format_version": 3,
  "file": "css/css-color/light-dark-currentcolor.html"
}
```

## style[0]

```css

.square {
  width: 100px;
  height: 100px;
  color: green;
  background-color: light-dark(currentColor, currentColor);
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
