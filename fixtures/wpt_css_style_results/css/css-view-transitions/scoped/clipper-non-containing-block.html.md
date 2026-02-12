# css/css-view-transitions/scoped/clipper-non-containing-block.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/clipper-non-containing-block.html"
}
```

## style[0]

```css

.clipper {
  overflow: clip;
  width: 100px;
  height: 100px;
}
#target {
  width: 200px;
  height: 300px;
  background: blue;
}
::view-transition-group(*) {
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
#target.after {
  background: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
