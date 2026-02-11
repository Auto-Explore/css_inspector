# css/css-page/page-box-004-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-box-004-print.html"
}
```

## style[0]

```css

  @page {
    size: 500px;
    margin: 5% 10% 15% 20%;
    padding: 20% 15% 10% 5%;
    border: 10px solid lightblue;
  }
  @page larger {
    size: 500px 800px;
  }
  @page smaller {
    size: 400px 300px;
  }
  :root {
    print-color-adjust: exact;
  }
  body {
    margin: 0;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
