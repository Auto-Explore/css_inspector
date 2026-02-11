# css/css-grid/subgrid/standalone-axis-size-006.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/standalone-axis-size-006.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  background: red;
  width: min-content;
}
.subgrid {
  display: grid;
  background: green;
  max-width: 100px;
  width: max-content;
  grid-template-rows: subgrid;
}
.w100 {
  display: inline-block;
  height: 50px;
  width: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
