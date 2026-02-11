# css/css-page/page-size-007-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-size-007-print.html"
}
```

## style[0]

```css

  @page {
    size: 200px 160px;
    margin: 0;
  }
  @page smaller {
    size: 160px 100px;
  }
  @page larger {
    size: 640px 400px;
  }
  :root {
    print-color-adjust: exact;
  }
  body {
    margin: 0;
  }
  .container {
    display: flow-root;
  }
  .smaller {
    page: smaller;
  }
  .larger {
    page: larger;
  }
  .float {
    float: left;
    width: 50px;
    height: 80px;
  }
  .smaller .float {
    width: 40px;
    height: 50px;
  }
  .larger .float {
    width: 160px;
    height: 200px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
