# css/css-view-transitions/set-current-time-transform.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/set-current-time-transform.html"
}
```

## style[0]

```css

:root { view-transition-name: unset; }
#target {
  width: 100px;
  height: 100px;
  background: blue;
  view-transition-name: target;
  position: relative;
}
.left {
  left: 0;
}
.right {
  left: 100px;
}

html::view-transition-group(*) {
  animation: unset;
}
html::view-transition-old(*) {
  animation: unset;
  opacity: 0;
}
@keyframes move {
  from {
    transform: translate(500px);
  }
}
html::view-transition-new(target) {
  animation-name: move;
  animation-duration: 1s;
  animation-timing-function: linear;
  animation-play-state: paused;
}
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
