# css/css-view-transitions/scroller.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scroller.html"
}
```

## style[0]

```css

#scroller {
  overflow: scroll;
  width: 100px;
  height: 100px;
  background: blue;
  view-transition-name: target;
}

#child {
  position: relative;
  width: 1000px;
  height: 1000px;
  background: green;
}

html::view-transition-group(root) { animation-duration: 300s; }
html::view-transition-old(target) {
  animation: unset;
  opacity: 1;
}
html::view-transition-new(target) {
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
