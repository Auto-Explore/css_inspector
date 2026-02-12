# css/css-break/table/table-fragmentation-003d-print.html

```json
{
  "format_version": 3,
  "file": "css/css-break/table/table-fragmentation-003d-print.html"
}
```

## style[0]

```css

@page { size: 5in 3in; margin: 0.5in; }
:root {
  print-color-adjust: exact;
}
body { margin: 0; }

table {
  inline-size: 3in;
  block-size: 3in;
  border-spacing: 0.125in;
  border: 0.25in solid black;
  padding: 0.125in;
  box-decoration-break: clone;
}
td {
  padding: 0;
}
thead td {
  block-size: 0.25in;
  background: blue;
}
tbody td {
  vertical-align: top;
  border: solid orange;
  border-width: 0.25in 0;
}
.content {
  block-size: 0.75in;
  background: gold;
}
tfoot td {
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
