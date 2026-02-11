# css/css-grid/alignment/grid-content-distribution-010.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-content-distribution-010.html"
}
```

## style[0]

```css

.block {
  position: absolute;
  z-index: -1;
  background: green;
  width: 200px;
  height: 200px;
}
.block > div {
  position: absolute;
  background: red;
  width: 15px;
  height: 15px;
}
.grid {
  z-index: 1;
  display: grid;
  width: 200px;
  height: 200px;
  grid-template-columns: 15px 15px 15px 15px;
  grid-template-rows: 15px 15px 15px 15px;
  grid-row-gap: 5px;
  grid-column-gap: 5px;
  align-content: space-evenly;
  justify-content: space-evenly;
}
.grid > div { background: green; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
