# css/css-page/page-box-005-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-box-005-print.html"
}
```

## style[0]

```css

  @page {
    size: 400px;
    margin: 50px;
    color: hotpink;
    border: 10px dotted;
    padding: 20px;
    background: yellow;
  }
  @page :first {
    color: orange;
    background: white;
  }
  @page :right {
    /* The first page in a horizontal writing mode LTR document is a right
       page. :first has higher specificity than :right, so these declarations
       are only for the third page. */
    color: black;
    background: cyan;
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
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
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
