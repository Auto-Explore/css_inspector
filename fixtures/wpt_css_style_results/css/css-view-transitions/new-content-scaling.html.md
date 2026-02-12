# css/css-view-transitions/new-content-scaling.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-scaling.html"
}
```

## style[0]

```css

.shared {
  view-transition-name: shared;
  contain: paint;
}
.src {
  position: relative;
  width: 100px;
  height: 100px;
}
.inner {
  position: absolute;
  inset: 20px;
  background: green;
}
.dst {
  position: relative;
  width: 500px;
  height: 500px;
  border: 5px solid blue;
  box-sizing: border-box;
}

html::view-transition-group(shared) {
  animation-delay: 500s;
}

html::view-transition-new(shared) {
  animation: unset;
  opacity: 1;
}
html::view-transition-old(shared) {
  animation: unset;
  opacity: 0;
}

html::view-transition-group(root) { animation: unset; opacity: 0; }
html::view-transition { background: lightpink; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
