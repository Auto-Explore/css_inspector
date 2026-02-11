# css/css-tables/rowspan-cell-border-after-color-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/rowspan-cell-border-after-color-ref.html"
}
```

## style[0]

```css

  table {
    border-collapse: collapse;
  }

  td {
    width: 100px;
    height: 50px;
    border: 1px solid black;
  }

  /* Row borders */
  #ref tr:nth-child(1) {
    border-bottom: 5px solid blue;
  }

  #ref tr:nth-child(2) {
    border-bottom: 5px solid green;
  }

  #ref tr:nth-child(3) {
    border-bottom: 5px solid red;
  }

  /* The rowspan cell should have red bottom border */
  #ref .rowspan-cell {
    border-bottom: 5px solid red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
