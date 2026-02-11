# css/css-pseudo/highlight-cascade/highlight-currentcolor-painting-properties-002.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-currentcolor-painting-properties-002.html"
}
```

## style[0]

```css

div {
  color: lime;
  background: green;
  margin: 10px;
}
::highlight(below) {
  color: yellow;
  background: maroon;
}
::highlight(empty) {
}
::highlight(color-currentcolor) {
  color: currentcolor;
}
::highlight(backgroundcolor-currentcolor) {
  background-color: currentcolor;
}
::highlight(textdecorationcolor-currentcolor) {
  text-decoration-line: underline;
  text-decoration-color: currentcolor;
}
::highlight(color-currentcolor-backgroundcolor-currentcolor) {
  color: currentcolor;
  background-color: currentcolor;
}
::highlight(color-currentcolor-backgroundcolor-blue) {
  color: currentcolor;
  background-color: blue;
}
::highlight(color-blue-backgroundcolor-currentcolor) {
  color: blue;
  background-color: currentcolor;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
