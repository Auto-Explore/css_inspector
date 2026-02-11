# css/css-transforms/preserve3d-and-flattening-z-order-003.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/preserve3d-and-flattening-z-order-003.html"
}
```

## style[0]

```css

div {
  position: absolute;
  height: 100px;
  width: 100px;
  top: 0;
  left: 0;
  background: red;
}
.outer {
  position: relative;
}
.outer, .sibling {
  transform-style: preserve-3d;
}
.child {
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
