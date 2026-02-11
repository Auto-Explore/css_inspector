# css/css-grid/grid-items/grid-size-with-orthogonal-child-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-size-with-orthogonal-child-002.html"
}
```

## style[0]

```css

.grid {
  display: inline-grid;
  grid-template-rows: 100px;
  background-color: green;
}

.item {
  writing-mode: vertical-lr;
}

.item > span {
  display: inline-block;
  inline-size: 100px;
  block-size: 50px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
