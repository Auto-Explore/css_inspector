# css/css-grid/placement/grid-placement-using-named-grid-lines-005.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/placement/grid-placement-using-named-grid-lines-005.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  width: 300px;
  height: 300px;
  grid-template-areas: "area";
  grid-template-columns: repeat(auto-fill, 100px);
  grid-template-rows: repeat(auto-fill, 100px) [area-end] 100px;
}
.grid > div {
  grid-area: area;
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
