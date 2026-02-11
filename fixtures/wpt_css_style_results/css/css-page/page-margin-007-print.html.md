# css/css-page/page-margin-007-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-margin-007-print.html"
}
```

## style[0]

```css

  @page {
    size: 400px 200px;
    margin: 0;
  }
  @page :first {
    margin-left: 50px;
  }
  @page :left {
    margin: 50px;
  }
  :root {
    print-color-adjust: exact;
  }
  body {
    margin: 0;
  }
  .container {
    width: 300px;
    background: gray;
  }
  .container > div {
    box-sizing: border-box;
    border: solid;
    width: 250px;
  }
  .left {
    height: 100px;
    background: hotpink;
  }
  .left::before {
    content: "Margins on every side.";
  }
  .right {
    height: 200px;
    background: cyan;
  }
  .right::before {
    content: "No page margins.";
  }
  .first {
    height: 200px;
    background: yellow;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
