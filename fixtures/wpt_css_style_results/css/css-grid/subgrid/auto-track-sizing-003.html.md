# css/css-grid/subgrid/auto-track-sizing-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/auto-track-sizing-003.html"
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
  display: grid;
  width: min-content;
  background: green;
  color: transparent;
  font: 25px/1 Ahem;
}
.subgrid {
  display: grid;
  grid-row: 1 / -1;
  grid-column: 1 / -1;
  grid-template: subgrid / subgrid;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
