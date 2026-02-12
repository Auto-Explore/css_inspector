# css/css-tables/table-intrinsic-size-004.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/table-intrinsic-size-004.html"
}
```

## style[0]

```css

.outer {
  width: min-content;
  background: green;
}
.table {
  writing-mode: vertical-rl;
  display: table;
  max-inline-size: 30px; /* Smaller than .content's inline size */
  block-size: 100px;
  border-inline: 10px solid green;
}
.content {
  inline-size: 80px;
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
