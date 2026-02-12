# css/css-view-transitions/hit-test-unpainted-element.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/hit-test-unpainted-element.html"
}
```

## style[0]

```css

html { view-transition-name: none }
div {
  width: 100px;
  height: 100px;
  contain: paint;
  view-transition-name: shared;
}
.before {
  position: relative;
  background: yellow;
  left: 200px;
}
.after {
  background: green;
}
.after:hover {
  background: red;
}

html::view-transition-group(shared) {
  animation-delay: 300s;
}
html::view-transition-old(shared) {
  animation: unset;
  opacity: 0;
}
html::view-transition-new(shared) {
  animation: unset;
  opacity: 1;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
