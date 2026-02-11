# css/css-grid/reference/grid-template-areas-must-keep-named-columns-order-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/reference/grid-template-areas-must-keep-named-columns-order-001-ref.html"
}
```

## style[0]

```css

.grid {
  display: inline-grid;
  background: grey;
  grid-template-columns: 50px 50px 50px 50px;
  grid-template-rows: 50px 50px;
}
.grid > :nth-child(1) { background: magenta; }
.grid > :nth-child(2) { background: blue; }
.grid > :nth-child(3) { background: yellow; }
.grid > :nth-child(4) { background: green; }
.grid > :nth-child(5) { background: black; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
