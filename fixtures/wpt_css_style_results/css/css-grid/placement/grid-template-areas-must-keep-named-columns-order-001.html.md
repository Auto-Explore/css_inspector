# css/css-grid/placement/grid-template-areas-must-keep-named-columns-order-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/placement/grid-template-areas-must-keep-named-columns-order-001.html"
}
```

## style[0]

```css

.grid {
  display: inline-grid;
  background: grey;
  grid-template-columns: [col] 50px [col] 50px [col] 50px [col] 50px;
  grid-auto-rows: 50px;
  grid-template-areas: "A . . .";
}
.grid > :nth-child(1) { background: magenta; }
.grid > :nth-child(2) { background: blue; }
.grid > :nth-child(3) { background: yellow; }
.grid > :nth-child(4) { background: green; }
.grid > :nth-child(5) { background: black; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
