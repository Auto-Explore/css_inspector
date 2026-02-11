# css/css-pseudo/highlight-cascade/highlight-currentcolor-painting-properties-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-currentcolor-painting-properties-002-ref.html"
}
```

## style[0]

```css

div {
  color: lime;
  background: green;
  margin: 10px;
}
div > span {
  color: yellow;
  background: maroon;
}
#empty > span > span {
}
#color-currentcolor > span > span {
  color: currentcolor;
}
#backgroundcolor-currentcolor > span > span {
  background-color: currentcolor;
}
#textdecorationcolor-currentcolor > span > span {
  text-decoration-line: underline;
  text-decoration-color: currentcolor;
}
#color-currentcolor-backgroundcolor-currentcolor > span > span {
  color: currentcolor;
  background-color: currentcolor;
}
#color-currentcolor-backgroundcolor-blue > span > span {
  color: currentcolor;
  background-color: blue;
}
#color-blue-backgroundcolor-currentcolor > span > span {
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
