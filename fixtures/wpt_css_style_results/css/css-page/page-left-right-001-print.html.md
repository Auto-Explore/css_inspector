# css/css-page/page-left-right-001-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-left-right-001-print.html"
}
```

## style[0]

```css

  @page {
    size: 600px 300px;
    margin: 0;
  }
  @page :left {
    margin-left: 500px;
    margin-bottom: 200px;
  }
  @page :right {
    margin-right: 500px;
    margin-top: 200px;
  }
  :root {
    print-color-adjust: exact;
  }
  body {
    margin: 0;
    background: yellow;
  }
  div {
    break-after: page;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
