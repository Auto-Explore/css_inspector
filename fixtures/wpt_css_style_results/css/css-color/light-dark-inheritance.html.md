# css/css-color/light-dark-inheritance.html

```json
{
  "format_version": 3,
  "file": "css/css-color/light-dark-inheritance.html"
}
```

## style[0]

```css

.square {
  width: 100px;
  height: 100px;
  color-scheme: dark;
  background-color: currentColor;
}
.container {
  color-scheme: light;
  color: light-dark(green, red);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
