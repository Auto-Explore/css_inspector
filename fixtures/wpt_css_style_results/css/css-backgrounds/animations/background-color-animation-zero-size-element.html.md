# css/css-backgrounds/animations/background-color-animation-zero-size-element.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-color-animation-zero-size-element.html"
}
```

## style[0]

```css

.container {
  animation: bgcolor 1s;
}
@keyframes bgcolor {
  0% { background-color: rgb(0, 200, 0); }
  100% { background-color: rgb(200, 0, 0); }
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
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
