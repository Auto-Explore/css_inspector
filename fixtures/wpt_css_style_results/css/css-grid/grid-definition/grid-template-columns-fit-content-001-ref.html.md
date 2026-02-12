# css/css-grid/grid-definition/grid-template-columns-fit-content-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/grid-template-columns-fit-content-001-ref.html"
}
```

## style[0]

```css

.grid {
  justify-content: start;
  width: 100px;
  position: relative;
  padding-top: 10px;
  margin-bottom: 5px;
  grid-column-gap: 5px;
}

.item {
  font: 10px/1 Ahem;
  background: cyan;
  padding-top: 2px;
}

.spanningItem {
  font: 10px/1 Ahem;
  grid-column: 1 / -1;
  grid-row: 2;
  background: salmon;
  padding-top: 2px;
}

.test {
  position: absolute;
  left: 0; right: 0; top: 0;
  height: 5px;
  background: purple;
}
.test:nth-child(2n) { background: orange; }

.floatLeft {
  float: left;
  width: 190px;
}

h3 { font-size: 1em; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
