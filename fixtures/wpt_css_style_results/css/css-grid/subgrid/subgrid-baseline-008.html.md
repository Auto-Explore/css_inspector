# css/css-grid/subgrid/subgrid-baseline-008.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/subgrid-baseline-008.html"
}
```

## style[0]

```css

.item {
  inline-size: 40px;
  box-sizing: border-box;
  border: solid 5px hotpink;
  line-height: 0;
  margin-block-start: 3px;
  margin-block-end: 5px;
}
.small {
  width: 20px;
  height: 20px;
  border: solid 5px cyan;
}
.first {
  align-self: baseline;
}
.last {
  align-self: last baseline;
}
.item.small.first {
  block-size: 50px;
}
.item.small.last {
  block-size: 100px;
}
span {
  width: 20px;
  height: 20px;
  box-sizing: border-box;
  border: solid 5px orange;
  display: inline-block;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
