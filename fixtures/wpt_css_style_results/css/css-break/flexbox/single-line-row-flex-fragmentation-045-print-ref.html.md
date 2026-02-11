# css/css-break/flexbox/single-line-row-flex-fragmentation-045-print-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/single-line-row-flex-fragmentation-045-print-ref.html"
}
```

## style[0]

```css

  @page { size: 5in 3in; margin: 0.5in; }
  :root {
    print-color-adjust: exact;
  }
  body {
    margin: 0;
  }
  .flexbox {
    display: block;
    border: 0.25in solid black;
  }
  .flexbox table {
    border-spacing: 0;
  }
  .flexbox thead, .flexbox tfoot {
    background: gold;
  }
  .text, tr {
    block-size: 0.25in;
  }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
