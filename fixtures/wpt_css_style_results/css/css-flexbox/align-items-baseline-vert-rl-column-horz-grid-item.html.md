# css/css-flexbox/align-items-baseline-vert-rl-column-horz-grid-item.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/align-items-baseline-vert-rl-column-horz-grid-item.html"
}
```

## style[0]

```css

#target {
  display: flex;
  flex-direction: column;
  writing-mode: vertical-rl;
  align-items: baseline;
  border: 3px solid black;
  font-family: monospace;
  font-size: 10px;
  line-height: 10px;
}
#target > :nth-child(1) {
  background: hotpink;
  writing-mode: horizontal-tb;
  padding-left: 30px;
  margin-left: 10px;
}
#target > :nth-child(2) {
  background: papayawhip;
  writing-mode: horizontal-tb;
}
.inner {
    display: grid;
    grid-template: auto / auto;
    border: 5px solid black;
    padding: 5px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
