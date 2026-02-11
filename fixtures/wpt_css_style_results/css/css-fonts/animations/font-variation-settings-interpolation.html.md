# css/css-fonts/animations/font-variation-settings-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/animations/font-variation-settings-interpolation.html"
}
```

## style[0]

```css

.parent {
  font-variation-settings: "test" 30;
}
.target {
  font-variation-settings: "test" 10;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “font-variation-settings”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-variation-settings”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
