# css/css-grid/subgrid/scrollbar-gutter-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/scrollbar-gutter-002.html"
}
```

## style[0]

```css

:root {
  overflow: scroll; /* Required to reproduce the bug on Firefox */
}
.grid {
  inline-size: 75px;
  display: inline-grid;
  border: 5px solid blue;
  vertical-align: top;
}
.subgrid {
  block-size: 75px;
  display: grid;
  grid: auto / subgrid;
}
.item {
  inline-size: 50px;
  block-size: 50px;
  background: green;
}
fieldset {
  margin: 0;
  padding: 0;
  border: 0;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
