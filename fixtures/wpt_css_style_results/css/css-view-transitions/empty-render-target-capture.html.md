# css/css-view-transitions/empty-render-target-capture.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/empty-render-target-capture.html"
}
```

## style[0]

```css


:root {
  view-transition-name: none;
}
.container {
  view-transition-name: container;
  width: 0px;
  height: 0px;
}
.child {
  view-transition-name: child;
  width: 100px;
  height: 100px;
  background: green;
  will-change: opacity;
}

::view-transition-group(*),
::view-transition-image-pair(*),
::view-transition-old(*),
::view-transition-new(*) {
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
