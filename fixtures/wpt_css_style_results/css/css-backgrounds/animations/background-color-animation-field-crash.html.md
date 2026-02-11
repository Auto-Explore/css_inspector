# css/css-backgrounds/animations/background-color-animation-field-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-color-animation-field-crash.html"
}
```

## style[0]

```css

@keyframes bgcolor {
  0% { background: Field; }
  100% { background: green; }
}
.target {
  animation: bgcolor 50ms;
  width: 100px;
  height: 100px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
