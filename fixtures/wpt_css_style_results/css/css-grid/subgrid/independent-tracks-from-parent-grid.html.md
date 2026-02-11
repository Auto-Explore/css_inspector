# css/css-grid/subgrid/independent-tracks-from-parent-grid.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/independent-tracks-from-parent-grid.html"
}
```

## style[0]

```css

.grid {
  display: inline-grid;
  background: red;
  gap: 25px;
}
.subgrid {
  display: grid;
  grid-template-columns: subgrid;
}
.item {
  background: green;
  height: 25px;
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
