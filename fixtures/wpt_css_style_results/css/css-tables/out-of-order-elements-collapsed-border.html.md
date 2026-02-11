# css/css-tables/out-of-order-elements-collapsed-border.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/out-of-order-elements-collapsed-border.html"
}
```

## style[0]

```css

table {
    border: 3px solid blue;
    border-collapse:collapse;}
td, th {height: 20px; width:20px;}
tbody {background-color: gold;}
thead {background-color: pink;}
tfoot {background-color: lightgreen;}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
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
