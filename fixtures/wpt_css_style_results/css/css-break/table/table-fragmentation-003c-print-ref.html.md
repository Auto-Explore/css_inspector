# css/css-break/table/table-fragmentation-003c-print-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-break/table/table-fragmentation-003c-print-ref.html"
}
```

## style[0]

```css

@page { size: 5in 3in; margin: 0.5in; }
:root {
  print-color-adjust: exact;
}
body { margin: 0; }

.table {
  inline-size: 3in;
  box-sizing: border-box;
  border: 0.25in solid black;
  padding: 0.125in 0.25in;
  box-decoration-break: clone;
}
.thead {
  block-size: 0.25in;
  background: blue;
}
.tbody {
  border: 0 solid orange;
  margin-block: 0.125in;
  background: gold;
}
.tfoot {
  block-size: 0.25in;
  background: purple;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
