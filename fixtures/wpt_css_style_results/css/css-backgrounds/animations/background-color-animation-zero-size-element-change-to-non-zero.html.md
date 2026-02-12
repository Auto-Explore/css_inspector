# css/css-backgrounds/animations/background-color-animation-zero-size-element-change-to-non-zero.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-color-animation-zero-size-element-change-to-non-zero.html"
}
```

## style[0]

```css

.container {
  animation: bgcolor 1000000s cubic-bezier(0,1,1,0) -500000s;
}
@keyframes bgcolor {
  0% { background-color: rgb(0, 200, 0); }
  100% { background-color: rgb(200, 0, 0); }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
