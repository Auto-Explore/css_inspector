# css/css-grid/alignment/grid-self-alignment-stretch-vertical-lr-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-self-alignment-stretch-vertical-lr-001.html"
}
```

## style[0]

```css

.grid {
  position: relative;
  display: inline-grid;
  grid-template-columns: 100px 150px;
  grid-template-rows: 150px 100px;
  font: 10px/1 Ahem;
  background: grey;
  writing-mode: vertical-lr;
}
.firstRowFirstColumn {
  grid-row: 1;
  grid-column: 1;
  background: green;
  justify-self: stretch;
  align-self: start;
}
.firstRowSecondColumn {
  grid-row: 1;
  grid-column: 2;
  background: blue;
  justify-self: start;
  align-self: stretch;
}
.secondRowFirstColumn {
  grid-row: 2;
  grid-column: 1;
  background: yellow;
  justify-self: start;
  align-self: start;
}
.secondRowSecondColumn {
  grid-row: 2;
  grid-column: 2;
  background: red;
  justify-self: stretch;
  align-self: stretch;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
