# css/css-grid/alignment/grid-content-distribution-with-collapsed-tracks-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-content-distribution-with-collapsed-tracks-001.html"
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
  width: 25px;
  height: 25px;
}
.grid {
  z-index: 1;
  display: grid;
  width: 200px;
  height: 200px;
  grid-template-columns: repeat(auto-fit, 25px);
  grid-template-rows: repeat(auto-fit, 25px);
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
