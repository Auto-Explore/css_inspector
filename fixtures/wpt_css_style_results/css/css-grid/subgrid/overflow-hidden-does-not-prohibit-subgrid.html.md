# css/css-grid/subgrid/overflow-hidden-does-not-prohibit-subgrid.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/overflow-hidden-does-not-prohibit-subgrid.html"
}
```

## style[0]

```css

.grid {
  display: inline-grid;
  grid-template: auto auto / auto auto;
  background-color: green;
}
.subgrid {
    grid-row: span 2;
    grid-column: 2;
    display: grid;
    grid-template-rows: subgrid;
    overflow: hidden;
}
.item {
    width: 50px;
    height: 50px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
