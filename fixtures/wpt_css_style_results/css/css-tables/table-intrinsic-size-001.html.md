# css/css-tables/table-intrinsic-size-001.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/table-intrinsic-size-001.html"
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
  inline-size: 30px; /* Smaller than .content's inline size */
  block-size: 100px;
  border-inline: 10px solid green;
  margin-inline: 10px;
}
.content {
  inline-size: 60px;
  height: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
