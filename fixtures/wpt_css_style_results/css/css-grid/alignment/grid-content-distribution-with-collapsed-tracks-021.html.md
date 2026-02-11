# css/css-grid/alignment/grid-content-distribution-with-collapsed-tracks-021.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-content-distribution-with-collapsed-tracks-021.html"
}
```

## style[0]

```css

.block {
  position: absolute;
  z-index: -1;
  background: green;
  width: 300px;
  height: 300px;
}
.block > div {
  position: absolute;
  background: red;
  width: 100px;
  height: 100px;
}
.grid {
  z-index: 1;
  display: grid;
  width: 300px;
  height: 300px;
  grid-template-columns: repeat(auto-fit, minmax(25px, auto));
  grid-template-rows: repeat(auto-fit, minmax(25px, auto));
  align-content: stretch;
  justify-content: stretch;
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
