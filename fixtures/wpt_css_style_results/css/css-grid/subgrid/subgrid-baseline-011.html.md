# css/css-grid/subgrid/subgrid-baseline-011.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/subgrid-baseline-011.html"
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
  grid-template-columns: repeat(2, auto);
}
div > div { grid-template-columns: subgrid }
.subgrid { grid-column: 1 / -1 }
.item:nth-child(2) { font-size: 30px }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
