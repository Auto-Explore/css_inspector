# css/css-view-transitions/capture-with-visibility-hidden-child.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/capture-with-visibility-hidden-child.html"
}
```

## style[0]

```css

.target {
  width: 100px;
  height: 100px;
  view-transition-name: target;
  background: blue;
}
.invisible {
  width: 500px;
  height: 500px;
  visibility: hidden;
}

::view-transition-group(root) {
  visibility: hidden;
  animation-duration: 500s;
}

::view-transtion-group(*),
::view-transtion-image-pair(*) {
  animation-play-state: paused;
}

::view-transition-old(*) {
  animation: unset;
  opacity: 1;
}

::view-transition-new(*) {
  animation: unset;
  opacity: 0;
}


::view-transition {
  background: pink;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
