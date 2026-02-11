# css/css-page/page-box-008-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-box-008-print.html"
}
```

## style[0]

```css

  html {
    writing-mode: vertical-rl;
  }
  @page {
    size: 400px 800px;
    margin-inline-start: 2%;
    margin-block-start: 8%;
    margin-inline-end: 6%;
    margin-block-end: 20%;
    padding-inline-start: 2%;
    padding-block-start: 8%;
    padding-inline-end: 6%;
    padding-block-end: 20%;
    background: blue;
  }
  :root {
    print-color-adjust: exact;
  }
  body {
    margin: 0;
    background: hotpink;
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
