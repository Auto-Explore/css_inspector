# css/css-grid/placement/grid-placement-using-named-grid-lines-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/placement/grid-placement-using-named-grid-lines-003.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  width: 300px;
  height: 300px;
  position: relative;
  top: -200px;
  left: -200px;
  grid-template-columns: repeat(auto-fill, 100px 100px) [area-start] 100px [area-end];
  grid-template-rows: repeat(auto-fill, 100px 100px [area-start]) [area-start] 100px [area-end];
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
