# css/css-flexbox/break-nested-float-in-flex-item-001-print.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/break-nested-float-in-flex-item-001-print.html"
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
  #flex {
    display: flex;
    flex-wrap: nowrap;
    flex-direction: column;
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
