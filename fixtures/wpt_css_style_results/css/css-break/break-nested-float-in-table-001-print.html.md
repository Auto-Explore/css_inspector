# css/css-break/break-nested-float-in-table-001-print.html

```json
{
  "format_version": 3,
  "file": "css/css-break/break-nested-float-in-table-001-print.html"
}
```

## style[0]

```css

  @page { size:5in 3in; margin:0.5in; }

  :root {
    print-color-adjust: exact;
  }
  html, body {
    margin: 0;
  }
  table {
    border-spacing: 0;
  }
  td {
    padding: 0;
  }
  #target {
    float: left;
    background: green;
    width: 2in;
    height: 6in; /* Expected to cover 3 pages. */
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
