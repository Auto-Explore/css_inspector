# css/css-flexbox/align-items-baseline-column-vert-lr-grid-item.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/align-items-baseline-column-vert-lr-grid-item.html"
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
  writing-mode: vertical-lr;
  padding-left: 30px;
  margin-left: 10px;
}
#target > :nth-child(2) {
  background: papayawhip;
  writing-mode: vertical-lr;
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
  "errors": 2,
  "messages": [
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
