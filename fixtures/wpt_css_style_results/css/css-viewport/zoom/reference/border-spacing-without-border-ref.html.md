# css/css-viewport/zoom/reference/border-spacing-without-border-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/reference/border-spacing-without-border-ref.html"
}
```

## style[0]

```css

body {
  --scale: 1;
}
.table {
  display: table;
}
.row {
  display: table-row;
}
.cell {
  display: table-cell;
  font-size: calc(1rem * var(--scale));
}
.spacing {
  border-collapse: separate;
  border-spacing: calc(17px * var(--scale)) calc(11px * var(--scale));
}
.zoom {
  --scale: 2;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
