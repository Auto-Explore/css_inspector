# css/css-break/flexbox/multi-line-row-flex-fragmentation-063-print.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/multi-line-row-flex-fragmentation-063-print.html"
}
```

## style[0]

```css

  @page { size: 5in 3in; margin: 0.5in; }
  :root {
    print-color-adjust: exact;
  }
  body { margin: 0; }

  .flexbox {
    display: flex;
    flex-wrap: wrap;
    border: 0.25in solid black;
    font-size: 0.25in;
    row-gap: 0.2in;
  }
  .item {
    contain: size;
    box-sizing: border-box;
    border: 4px solid purple;
    height: 0.5in;
    flex: 0 0 100%;
  }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
