# css/css-transforms/preserve3d-and-flattening-z-order-008.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/preserve3d-and-flattening-z-order-008.html"
}
```

## style[0]

```css

div, span {
  height: 100px;
  width: 100px;
  background: red;
}
span {
  display: inline-block;
  vertical-align: bottom;
}
.outer, .sibling {
  transform-style: preserve-3d;
}
.sibling {
  margin-top: -100px;
  transform: translateZ(-10px);
}

.flattener:first-child {
  background: linear-gradient(to bottom, green 0%, green 25%, red 25%, red 100%);
}
.flattener:first-child > .child {
  background: linear-gradient(to bottom, green 0%, green 50%, red 50%, red 100%);
  margin-top: 50px;
  height: 50px;
}
.flattener:last-child {
  background: linear-gradient(to bottom, green 0px, green 25px, red 25px, red 75px);
  margin-top: -75px;
  height: 75px;
}
.flattener:last-child > .child {
  background: green;
  margin-top: 50px;
  height: 25px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
