# css/css-backgrounds/animations/background-color-animation-will-change-contents.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-color-animation-will-change-contents.html"
}
```

## style[0]

```css

.container {
  width: 600px;
  height: 600px;
  will-change: contents;
  /* Start with a short delay and ensure that we pick up the color change when
     the animation enters the active phase. */
  animation: bgcolor 10s steps(1, jump-start) backwards 0.2s;
}
@keyframes bgcolor {
  0% { background-color: rgba(200, 0, 0, 1); }
  100% { background-color: rgba(0, 200, 0, 1); }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
