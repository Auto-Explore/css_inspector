# css/css-break/table/table-fragmentation-001a-print.html

```json
{
  "format_version": 3,
  "file": "css/css-break/table/table-fragmentation-001a-print.html"
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
  border-spacing: 0.25in;
  border: 0.25in solid black;
  padding: 0.25in;
}
td {
  vertical-align: top;
  padding: 0;
  border: 0.25in solid orange;
}
.content {
  block-size: 1.5in;
  background: gold;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
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
