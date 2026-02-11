# css/css-page/basic-pagination-003-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/basic-pagination-003-print.html"
}
```

## style[0]

```css

  @page {
    size: 293px;
    margin: 5px;
  }
  :root {
    print-color-adjust: exact;
  }
  body {
    margin: 0;
    background: yellow;
  }
  div {
    /* Forced breaks defeat break avoidance hints. */
    break-before: page;
    break-after: avoid;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
