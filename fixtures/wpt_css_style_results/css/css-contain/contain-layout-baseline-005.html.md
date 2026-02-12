# css/css-contain/contain-layout-baseline-005.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-baseline-005.html"
}
```

## style[0]

```css

.wrapper {
  height: 110px;
}
.wrapper > * {
  contain: layout;
  background: cyan;
  font-size: 20px;
  vertical-align: baseline;
}
.wrapper > :nth-child(1) {
  background: magenta;
}
.inline-block {
  display: inline-block;
}
canvas {
  width: 100px;
  height: 100px;
}
fieldset, details {
  display: inline-block;
  width: max-content;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
