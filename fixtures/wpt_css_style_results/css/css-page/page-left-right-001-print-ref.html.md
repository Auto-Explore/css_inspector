# css/css-page/page-left-right-001-print-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-left-right-001-print-ref.html"
}
```

## style[0]

```css

  @page {
    size: 600px 300px;
    margin: 0;
  }
  :root {
    print-color-adjust: exact;
  }
  body {
    margin: 0;
  }
  div {
    break-after: page;
    width: 100px;
    height: 100px;
    overflow: clip; /* Depending on font, glyphs may overflow. */
    background: yellow;
  }
  div:nth-child(odd) {
    margin-top: 200px;
  }
  div:nth-child(even) {
    margin-left: 500px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
