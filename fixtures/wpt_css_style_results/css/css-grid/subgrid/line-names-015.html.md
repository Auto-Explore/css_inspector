# css/css-grid/subgrid/line-names-015.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/line-names-015.html"
}
```

## style[0]

```css

.grid {
  background: green;
  display: grid;
  height: 100px;
  width: 100px;
  grid-template: 50px 50px / 50px 50px;
  grid-template-areas: "item item"
                       "item item";
}
.subgrid {
  display: grid;
  grid-template-columns: subgrid;
  grid-template-rows: 50px 50px;
  grid-column: span 2;
  grid-row: span 2;
}
.item {
  background: red;
  grid-area: item;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “grid-template-areas”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
