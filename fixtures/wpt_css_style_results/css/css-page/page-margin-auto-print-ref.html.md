# css/css-page/page-margin-auto-print-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-margin-auto-print-ref.html"
}
```

## style[0]

```css

  @page {
    size: 20em 7em;
    margin: 0;
  }
  :root {
    print-color-adjust: exact;
  }
  .pagebox {
    break-before: page;
    display: flex;
    width: 20em;
    height: 7em;
  }
  .pagebox > div {
    width: 12em;
    height: 3em;
    margin: auto;
    background: yellow;
  }
  body {
    margin: 0;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
