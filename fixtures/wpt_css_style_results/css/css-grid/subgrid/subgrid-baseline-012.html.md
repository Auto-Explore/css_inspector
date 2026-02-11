# css/css-grid/subgrid/subgrid-baseline-012.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/subgrid-baseline-012.html"
}
```

## style[0]

```css

.grid {
  display: inline-grid;
  background: red;
}
.subgrid {
  display: grid;
  grid-template-rows: subgrid;
  writing-mode: vertical-rl;
}
.item {
  align-self: baseline;
  background: green;
  height: 100px;
  width: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
