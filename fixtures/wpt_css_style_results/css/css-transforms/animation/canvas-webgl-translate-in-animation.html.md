# css/css-transforms/animation/canvas-webgl-translate-in-animation.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/canvas-webgl-translate-in-animation.html"
}
```

## style[0]

```css

@keyframes move {
  to { transform: translate(300px); }
}

canvas {
  will-change: transform;
  animation: move;
  animation-duration: 1s;
  animation-timing-function: linear;
  animation-play-state: paused;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
