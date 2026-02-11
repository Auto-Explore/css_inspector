# css/css-animations/animation-delay-010.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-delay-010.html"
}
```

## style[0]

```css

@keyframes all-orange {
  from { background-color: orange }
  to   { background-color: orange }
}
div {
  width: 100px;
  height: 100px;
  background-color: red;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
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
