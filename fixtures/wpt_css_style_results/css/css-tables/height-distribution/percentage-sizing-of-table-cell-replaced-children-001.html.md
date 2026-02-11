# css/css-tables/height-distribution/percentage-sizing-of-table-cell-replaced-children-001.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/height-distribution/percentage-sizing-of-table-cell-replaced-children-001.html"
}
```

## style[0]

```css

.table {
  display: table;
  border: solid 5px black;
  width: 150px;
  height: 100px;
}

.td {
  display: table-cell;
  background: cyan;
  overflow: scroll;
  padding: 5px 15px 10px 20px;
  border: solid magenta;
  border-width: 12px 9px 6px 3px;
}

img {
  display: block;
  background: yellow;
  width: 100%;
  height: 100%;
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
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
