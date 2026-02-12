# css/css-grid/subgrid/contribution-size-flex-tracks-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/contribution-size-flex-tracks-001.html"
}
```

## style[0]

```css

.grid {
  background: red;
  display: inline-grid;
  grid-template: repeat(4, minmax(0, 1fr)) / repeat(10, minmax(0, 1fr));
}
.subgrid {
  width: 50px;
  height: 50px;
  background: green;
  display: grid;
  grid-row: 1 / -1;
  grid-column: 1 / -1;
  grid-template: subgrid / subgrid;
}
.item {
  width: 10px;
  height: 25px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
