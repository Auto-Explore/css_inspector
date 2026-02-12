# css/css-grid/grid-lanes/tentative/subgrid/row/grid-lanes-subgrid-002f.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/subgrid/row/grid-lanes-subgrid-002f.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:10px/1 monospace; padding:0; margin:0;
}

grid {
  display: inline-grid-lanes;
  grid-lanes-direction: row;
  grid-template-rows: 40px 30px 20px;
  gap: 4px 2px;
  padding: 1px 3px 5px 7px;
  border: solid;
  border-width: 3px 5px 1px 1px;
  background: lightgrey content-box;
}
.rows {
  grid-lanes-direction: column;
  grid-template-columns: 40px 30px 20px;
}
item {
  background: grey;
  width: 3ch;
  position: relative;
}
item:nth-child(2n) { background:purple; width:auto; }
item:nth-child(1) {
  border: solid;
  border-width: 3px 5px 1px 1px;
  margin: 7px 1px 5px 3px;
}
subgrid {
  display: grid;
  grid-row: auto/span 2;
  grid-column: auto/span 2;
  grid: subgrid / subgrid;
  grid-gap: 6px 8px;
  background: yellow;
}
subgrid.definite {
  grid-row-start:2;
}
subgrid.extent {
  grid-row: auto/span 3;
}
.rows > subgrid.definite {
  grid-column-start:2;
}
.rows > subgrid.extent {
  grid-row: auto/span 2;
  grid-column: auto/span 3;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
