# css/css-tables/table-cell-overflow-explicit-height-002.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/table-cell-overflow-explicit-height-002.html"
}
```

## style[0]

```css

td {
  height: 20px;
  max-height: 20px;
  border: 2px solid cyan;
  overflow: hidden;
}
.tall {
  height: 300px;
  background: blue;
  border: 2px solid black;
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
