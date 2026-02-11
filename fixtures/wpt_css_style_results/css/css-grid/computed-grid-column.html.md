# css/css-grid/computed-grid-column.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/computed-grid-column.html"
}
```

## style[0]

```css

.grid {
  position: relative;
  display: inline-grid;
  grid-template-columns: 100px 150px;
  grid-template-rows: 150px 100px;
}
.firstRowSecondColumn {
  grid-row: calc(2 * sign(100em - 1px) - 1);
  grid-column: calc(3 - sign(100em - 1px));
}
.someHugeValue {
  /* Just check that we don't crash. */
  grid-row: calc(1e100);
  grid-column: calc(-1e100);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
