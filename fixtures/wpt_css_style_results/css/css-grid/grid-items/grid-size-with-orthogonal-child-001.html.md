# css/css-grid/grid-items/grid-size-with-orthogonal-child-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-size-with-orthogonal-child-001.html"
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
  writing-mode: vertical-rl;
  inline-size: 100%;
  block-size: 50%;
}

.item > span {
  display: inline-block;
  inline-size: 40px;
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
