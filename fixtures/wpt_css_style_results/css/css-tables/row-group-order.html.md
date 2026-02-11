# css/css-tables/row-group-order.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/row-group-order.html"
}
```

## style[0]

```css

  table {
    font: 8px Ahem;
    border-collapse: collapse;
  }
  thead, tbody, tfoot {
    border: 10px solid;
  }
  .first {
    border-color: green;
  }
  .second {
    border-color: blue;
  }
  .third {
    border-color: yellow;
  }
  tbody {
    border-color: orange;
  }
  td {
    width: 50px;
    height: 50px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
