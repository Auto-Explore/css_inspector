# css/css-grid/subgrid/standalone-axis-size-007.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/standalone-axis-size-007.html"
}
```

## style[0]

```css

.grid {
  background: red;
  display: inline-grid;
}
.subgrid {
  display: grid;
  grid-row: span 2;
  background: green;
  min-width: min-content;
  grid-auto-flow: column;
  grid-template-rows: subgrid;
}
.w100 {
  height: 50px;
  width: 100px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
