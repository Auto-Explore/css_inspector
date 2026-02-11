# css/css-grid/subgrid/line-names-013.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/line-names-013.html"
}
```

## style[0]

```css

.grid {
  background: red;
  display: grid;
  grid-template: 100px 100px / 100px;
  height: 100px;
  width: 100px;
}
.subgrid {
  display: grid;
  grid-template-areas: "item item"
                       "item item";
  grid-template-rows: subgrid;
}
.item {
  background: green;
  grid-area: item;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-template-areas”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
