# css/css-grid/subgrid/standalone-axis-size-004.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/standalone-axis-size-004.html"
}
```

## style[0]

```css

.grid {
  display: inline-grid;
  grid-template: 100px / minmax(auto, 100px);
}
.subgrid {
  width: 100%;
  display: grid;
  background: green;
  grid-template-rows: subgrid;
}
.w200 { width: 200px }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
