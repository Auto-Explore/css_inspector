# css/css-backgrounds/animations/two-background-color-animation-diff-length2.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/two-background-color-animation-diff-length2.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
  background-color: green;
  animation: bgcolor2 10s steps(2, end), bgcolor1 2s 1s steps(2, end);
}
@keyframes bgcolor1 {
  0% { background-color: rgb(0, 200, 0); }
  100% { background-color: rgb(200, 0, 0); }
}
@keyframes bgcolor2 {
  0% { background-color: rgb(0, 0, 199); }
  100% { background-color: rgb(0, 0, 200); }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
