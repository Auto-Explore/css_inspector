# css/css-tables/table-intrinsic-size-002.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/table-intrinsic-size-002.html"
}
```

## style[0]

```css

.outer {
  width: min-content;
  background: green;
}
.table {
  display: table;
  max-inline-size: 30px; /* Smaller than .content's inline size */
  block-size: 100px;
  border-inline: 10px solid green;
  margin-inline: 10px;
}
.content {
  inline-size: 60px;
  block-size: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
