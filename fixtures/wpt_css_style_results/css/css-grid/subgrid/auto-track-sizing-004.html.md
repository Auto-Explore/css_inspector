# css/css-grid/subgrid/auto-track-sizing-004.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/auto-track-sizing-004.html"
}
```

## style[0]

```css

.wrapper {
  width: 100px;
  height: 100px;
  background: red;
}
.grid {
  display: inline-grid;
  grid-template-columns: repeat(10, auto);
  grid-template-rows: repeat(5, 5px auto);
}
.subgrid {
  display: grid;
  grid-template: subgrid / subgrid;
  grid-column: 1 / -1;
  grid-row: 1 / -1;
}
.item {
  background: green;
  height: 50px;
  width: 50px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
