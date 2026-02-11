# css/css-animations/animation-delay-009.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-delay-009.html"
}
```

## style[0]

```css

@keyframes all-red {
  from { background-color: red }
  to   { background-color: red }
}
div {
  width: 100px;
  height: 100px;
  background-color: orange;
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
