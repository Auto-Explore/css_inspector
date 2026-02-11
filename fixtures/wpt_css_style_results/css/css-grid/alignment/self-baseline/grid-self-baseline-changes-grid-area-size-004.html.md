# css/css-grid/alignment/self-baseline/grid-self-baseline-changes-grid-area-size-004.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/self-baseline/grid-self-baseline-changes-grid-area-size-004.html"
}
```

## style[0]

```css

.block, .grid { font: 10px/1 Ahem; }
.big { font-size: 20px; }
.block {
  position: absolute;
  z-index: -1;
  background: green;
  width: 100px;
  height: 100px;
}
.block > div {
  position: absolute;
  color: red;
}
.grid {
  display: inline-grid;
  color: green;
  grid-auto-columns: 50px;
  align-items: baseline;
  align-content: start;
  vertical-align: top;
}
.firstRowFirstColumn {
  grid-row: 1;
  grid-column: 1;
}
.firstRowSecondColumn {
  grid-row: 1;
  grid-column: 2;
}
.secondRowFirstColumn {
  grid-row: 2;
  grid-column: 1;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
