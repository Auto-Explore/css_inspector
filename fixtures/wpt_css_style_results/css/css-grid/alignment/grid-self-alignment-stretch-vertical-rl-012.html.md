# css/css-grid/alignment/grid-self-alignment-stretch-vertical-rl-012.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-self-alignment-stretch-vertical-rl-012.html"
}
```

## style[0]

```css

.grid {
  position: relative;
  display: inline-grid;
  font: 20px/1 Ahem;
  background: grey;
  width: 250px;
  height: 250px;
  writing-mode: vertical-rl;
}
.firstRowFirstColumn {
  grid-row: 1;
  grid-column: 1;
  background: green;
  justify-self: stretch;
  align-self: start;
  padding-bottom: 10px;
}
.firstRowSecondColumn {
  grid-row: 1;
  grid-column: 2;
  background: blue;
  justify-self: start;
  align-self: stretch;
  padding-left: 20px;
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
  padding-bottom: 20px;
  padding-left: 10px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
