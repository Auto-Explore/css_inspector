# css/css-backgrounds/animations/background-color-animation-non-empty-no-draw-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-color-animation-non-empty-no-draw-crash.html"
}
```

## style[0]

```css

@keyframes bgcolor {

  0% { background: blue; }
  100% { background: none; }
}
.target {
  animation: bgcolor 50ms;
  opacity: 0.9;
  height: 0.4px;
  margin: 21.6px;
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
