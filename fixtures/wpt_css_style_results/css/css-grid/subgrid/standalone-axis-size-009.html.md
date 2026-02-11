# css/css-grid/subgrid/standalone-axis-size-009.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/standalone-axis-size-009.html"
}
```

## style[0]

```css

.grid {
  display: inline-grid;
  grid-template: minmax(auto, 100px) / 100px;
}
.subgrid {
  height: 100%;
  display: grid;
  background: green;
  grid-template-columns: subgrid;
}
.h200 { height: 200px }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
