# css/css-view-transitions/nested/rotated-container.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/rotated-container.html"
}
```

## style[0]

```css

:root { view-transition-name: none }
::view-transition { background: pink }
::view-transition-group(*) {
  animation-duration: 1s;
  animation-play-state: paused;
}
::view-transition-old(*) {
  animation: unset;
  opacity: 0;
}
::view-transition-new(*) {
  animation: unset;
  opacity: 1;
}

#target {
  width: 100px;
  height: 100px;
  position: absolute;
  left: 50px;
  top: 50px;
  background: lightblue;

  view-transition-name: container;
  view-transition-group: contain;
}
.after {
  transform: translateX(100px) rotate(45deg);
}
#item {
  view-transition-name: item;
  width: 50px;
  height: 50px;
  background: blue;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
