# css/css-flexbox/align-items-baseline-column-vert.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/align-items-baseline-column-vert.html"
}
```

## style[0]

```css

#target {
  writing-mode: vertical-rl;
  display: flex;
  flex-direction: column;
  align-items: baseline;
  inline-size: 200px;
  border: solid 3px;
}
#target > div {
  margin: 5px 7px 9px 11px;
}
#target > :nth-child(1) {
  background: lime;
  font: 30px/1 Ahem;
  writing-mode: horizontal-tb;
}
#target > :nth-child(2) {
  background: hotpink;
  font: 20px/1 Ahem;
  writing-mode: horizontal-tb;
}
#target > :nth-child(3) {
  background: cyan;
  font: 16px/1 Ahem;
  width: 40px;
  height: 40px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
