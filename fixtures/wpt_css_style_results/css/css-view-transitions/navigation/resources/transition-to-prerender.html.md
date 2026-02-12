# css/css-view-transitions/navigation/resources/transition-to-prerender.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/resources/transition-to-prerender.html"
}
```

## style[0]

```css

@view-transition { navigation: auto; }
html { background: red; }
div {
  width: 200px;
  height: 200px;
  background: red;
  color: black;
  position: absolute;
  top: 40px;
  right: 8px;
  view-transition-name: target;
}
::view-transition {
  background: lightblue;
}
::view-transition-group(root) {
  visibility: hidden;
  animation-duration: 500s;
}
::view-transition-group(target) {
  animation-play-state: paused;
}
::view-transition-new(target) {
  animation: unset;
  opacity: 1;
}
::view-transition-old(target) {
  animation: unset;
  opacity: 0;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
