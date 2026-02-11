# css/css-transforms/preserve3d-and-flattening-z-order-006.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/preserve3d-and-flattening-z-order-006.html"
}
```

## style[0]

```css

body {
  margin: 0;
}
div, span {
  height: 100px;
  width: 100px;
  background: red;
}
span {
  display: inline-block;
  vertical-align: top;
}
div:not(:first-child):not(.outer) {
  /* put everything at the same position without using absolute positioning */
  margin-top: -100px;
}
.outer, .sibling {
  transform-style: preserve-3d;
}
.outer {
  background: gray;
}
.flattener {
  /* adding position:relative or a transform changes things */
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
