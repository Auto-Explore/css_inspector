# css/css-view-transitions/pseudo-element-overflow-clip-with-border-radius-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-element-overflow-clip-with-border-radius-ref.html"
}
```

## style[0]

```css

body {
  margin: 0px;
}
div {
  width: 200px;
  height: 200px;
}
#target {
  position: absolute;
  width: 200px;
  height: 200px;
  background: green;
  overflow: clip;
  overflow-clip-margin: 30px;
  border-bottom-right-radius: 30px;
}
#inner {
  position: relative;
  left: 100px;
  top: 100px;
  background: blue;
}
.offset {
  left: 400px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
