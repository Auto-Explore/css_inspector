# css/css-grid/alignment/grid-content-distribution-with-collapsed-tracks-024.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-content-distribution-with-collapsed-tracks-024.html"
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
  width: 60px;
  height: 60px;
}
.grid {
  z-index: 1;
  display: grid;
  width: 300px;
  height: 300px;
  grid-template-columns: repeat(auto-fit, minmax(25px, auto));
  grid-template-rows: repeat(auto-fit, minmax(25px, auto));
  grid-row-gap: 20px;
  grid-column-gap: 20px;
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
