# css/css-grid/subgrid/dynamic-subgridded-item-height.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/dynamic-subgridded-item-height.html"
}
```

## style[0]

```css

.wrapper {
  background: red;
  height: 100px;
  width: 100px;
}
.grid {
  display: grid;
  grid-template-columns: 100px;
}
.subgrid {
  display: grid;
  grid-template-columns: subgrid;
  overflow: clip;
}
.container { background: green }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
