# css/css-grid/subgrid/percentage-track-sizing.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/percentage-track-sizing.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  background: red;
  height: 100px;
  width: 100px;
}
.subgrid {
  display: grid;
  grid-template-columns: 100% 0%;
  grid-template-rows: subgrid;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
