# css/css-page/page-name-unnamed-trailing-001-print-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-name-unnamed-trailing-001-print-ref.html"
}
```

## style[0]

```css

@page{
    size: 200px 300px;
}
@page a{
    margin: 0;
}
@page b{
    /* Add a margin so that even if page-orientation doesn't work in the tests
     * the pages are different. */
    margin: 20px;
    page-orientation: rotate-left;
}
:root {
  print-color-adjust: exact;
}
body{
    margin: 0;
}
.page{
    break-after: page;
    overflow: hidden;
    width: 160px;
    height: 260px;
}
.marker{
    width: 90px;
    height: 90px;
    background-color: cyan;
}
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “page-orientation”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
