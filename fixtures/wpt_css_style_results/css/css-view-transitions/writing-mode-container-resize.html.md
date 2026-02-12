# css/css-view-transitions/writing-mode-container-resize.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/writing-mode-container-resize.html"
}
```

## style[0]

```css

#target {
  view-transition-name: target;
  background: lightblue;
  width: 100px;
  height: 100px;
}
.vertical {
  writing-mode: vertical-lr;
}
::view-transition-group(root) {
  visibility: hidden;
  animation-duration: 500s;
}
::view-transition-old(target) {
  animation: unset;
  opacity: 1;
}
::view-transition-new(target) {
  animation: unset;
  opacity: 0;
}
::view-transition-group(target) {
  height: 50px;
  border: 1px solid black;
  animation: unset;
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
