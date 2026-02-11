# css/css-backgrounds/animations/background-color-animation-with-table3.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-color-animation-with-table3.html"
}
```

## style[0]

```css

table {
  width: 160px;
}
td {
  animation: bgcolor 100s;
}
@keyframes bgcolor {
  0% { background-color: rgba(0, 200, 0, 0); }
  100% { background-color: rgba(200, 0, 0, 0); }
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
