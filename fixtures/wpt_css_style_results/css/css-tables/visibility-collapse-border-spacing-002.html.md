# css/css-tables/visibility-collapse-border-spacing-002.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/visibility-collapse-border-spacing-002.html"
}
```

## style[0]

```css

table {
  border-spacing: 50px 100px;
  background: green;
}
td {
  width: 100px;
  padding: 0;
  background: red;
}
tr + tr {
  visibility: collapse;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
