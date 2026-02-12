# css/css-view-transitions/new-content-changes-overflow-left.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-changes-overflow-left.html"
}
```

## style[0]

```css

#target {
  position: relative;
  background: green;
  left: 10px;
  width: 100px;
  height: 100px;
  view-transition-name: target;
}
#target.toggle {
  outline: 300px solid transparent;
}

html::view-transition-group(*) { animation-duration: 300s; }
html::view-transition-new(*) { animation: unset; opacity: 1; }
html::view-transition-old(*) { animation: unset; opacity: 0; }
html::view-transition-group(root) { animation: unset; opacity: 0; }
html::view-transition { background: pink; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
