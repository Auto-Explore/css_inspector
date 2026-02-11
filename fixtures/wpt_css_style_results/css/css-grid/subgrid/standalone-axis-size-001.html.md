# css/css-grid/subgrid/standalone-axis-size-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/standalone-axis-size-001.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  grid-template-rows: 100px;
}
.subgrid {
  display: grid;
  background: red;
  width: min-content;
  grid-template-rows: subgrid;
}
.item {
  background: green;
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
