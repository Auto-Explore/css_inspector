# css/css-break/break-inside-avoid-multicol-001-print.html

```json
{
  "format_version": 3,
  "file": "css/css-break/break-inside-avoid-multicol-001-print.html"
}
```

## style[0]

```css

  @page {
    size: 5in 3in;
    margin: 0.5in;
  }
  :root {
    print-color-adjust: exact;
  }
  body {
    margin: 0;
  }
  article {
    column-count: 2;
    column-gap: 0;
  }
  article > div {
    width: 100%;
    height: 3in;
    background: green;
    break-inside: avoid;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
