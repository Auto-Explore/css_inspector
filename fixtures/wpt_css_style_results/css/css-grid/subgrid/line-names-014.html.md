# css/css-grid/subgrid/line-names-014.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/line-names-014.html"
}
```

## style[0]

```css

.grid {
  background: red;
  display: grid;
  height: 100px;
  width: 100px;
  grid-template: 50px 50px / 50px 50px;
}
.subgrid {
  display: grid;
  grid-template-columns: subgrid;
  grid-template-rows: 50px 50px;
  grid-column: span 2;
  grid-row: span 2;
  grid-template-areas: "item item"
                       "item item";
}
.item {
  background: green;
  grid-area: item;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
