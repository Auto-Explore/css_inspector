# css/css-backgrounds/animations/background-color-animation-element-not-visible-at-current-viewport.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-color-animation-element-not-visible-at-current-viewport.html"
}
```

## style[0]

```css

.box {
  width: 100px;
  height: 10000px;
}
.container {
  width: 50px;
  height: 50px;
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
