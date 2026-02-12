# css/css-flexbox/align-items-baseline-column-vert-rl-table-item.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/align-items-baseline-column-vert-rl-table-item.html"
}
```

## style[0]

```css

#target {
  display: flex;
  flex-direction: column;
  width: 200px;
  align-items: baseline;
  border: 3px solid black;
  font-family: monospace;
  font-size: 10px;
  line-height: 10px;
}
#target > :nth-child(1) {
  background: hotpink;
  writing-mode: vertical-rl;
  padding-left: 30px;
  margin-left: 10px;
}
#target > :nth-child(2) {
  background: papayawhip;
  writing-mode: vertical-rl;
}
.inner {
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
