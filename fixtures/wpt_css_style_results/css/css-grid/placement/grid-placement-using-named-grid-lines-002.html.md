# css/css-grid/placement/grid-placement-using-named-grid-lines-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/placement/grid-placement-using-named-grid-lines-002.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  width: 100px;
  height: 100px;
  background: red;
  grid-template-columns: [area-start] repeat(auto-fill, 10px) [area-end];
  grid-template-rows: [area-start] repeat(auto-fill, 10px [area-start]) [area-end];
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
