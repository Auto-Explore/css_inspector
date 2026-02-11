# css/css-grid/subgrid/subgrid-baseline-010.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/subgrid-baseline-010.html"
}
```

## style[0]

```css

body {
  margin: 0;
  padding: 0;
  font: 15px/1 Ahem;
}
div {
  align-items: baseline;
  display: inline-grid;
  grid-template: 30px / 60px;
}
.subgrid { grid-template: subgrid / repeat(2, 30px) }
.item { grid-template: auto / subgrid }
.item:nth-child(2) { font-size: 30px }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
