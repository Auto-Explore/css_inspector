# css/css-grid/abspos/empty-grid-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/empty-grid-001.html"
}
```

## style[0]

```css

.gridWithAbsolutePositionedItem {
  /* Ensures that the grid container is the containing block of the absolutely positioned grid children. */
  position: relative;
}

.grid {
  grid-auto-columns: 200px;
  grid-auto-rows: 200px;
}

.item {
  position: absolute;
  font: 10px/1 Ahem;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
