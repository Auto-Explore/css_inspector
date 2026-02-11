# css/css-tables/fixed-layout-excess-width-distribution-001.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/fixed-layout-excess-width-distribution-001.html"
}
```

## style[0]

```css

table {
  width: 300px;
  border-collapse: collapse;
  table-layout: fixed;
  height: 20px;
}

td {
  padding: 0px;
  background: lime;
  outline: 1px solid blue;
}

td:nth-child(1) { width: 20px; }
td:nth-child(2) { width: 10px; }
td:nth-child(3) { width: 10%; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
