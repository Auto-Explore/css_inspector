# css/css-view-transitions/set-current-time-transform-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/set-current-time-transform-ref.html"
}
```

## style[0]

```css

@keyframes move {
  from {
    transform: translate(500px);
  }
}
#target {
  width: 100px;
  height: 100px;
  background: blue;
  view-transition-name: target;
  position: relative;
  left: 100px;

  animation-name: move;
  animation-duration: 1s;
  animation-timing-function: linear;
  animation-play-state: paused;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
