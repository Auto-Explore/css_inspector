# css/css-backgrounds/animations/background-color-animation-non-zero-size-element-change-to-zero.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-color-animation-non-zero-size-element-change-to-zero.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
  animation: bgcolor 1000000s cubic-bezier(0,1,1,0) -500000s;
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
