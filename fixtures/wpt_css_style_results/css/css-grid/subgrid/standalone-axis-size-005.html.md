# css/css-grid/subgrid/standalone-axis-size-005.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/standalone-axis-size-005.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  background: red;
  width: max-content;
  grid-template-columns: minmax(min-content, max-content);
}
.subgrid {
  display: grid;
  background: green;
  width: min-content;
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
