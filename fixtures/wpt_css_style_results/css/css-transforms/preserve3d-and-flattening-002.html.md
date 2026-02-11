# css/css-transforms/preserve3d-and-flattening-002.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/preserve3d-and-flattening-002.html"
}
```

## style[0]

```css

body {
  margin: 0;
}
div {
  position: absolute;
  height: 100px;
  width: 100px;
  top: 0;
  left: 0;
}
.outer, .sibling {
  transform-style: preserve-3d;
}
.outer {
  background: gray;
}
.flattener {
  background: fuchsia;
}
.sibling {
  background: blue;
  transform: translate3d(25px, 50px, -20px);
}
.child {
  background: silver;
  transform: translate3d(50px, 0, 10px);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
