# css/css-grid/alignment/grid-content-distribution-004.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-content-distribution-004.html"
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
  width: 100px;
  height: 100px;
}
.grid {
  z-index: 1;
  display: grid;
  width: 200px;
  height: 200px;
  grid-template-columns: auto auto;
  grid-template-rows: auto auto;
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
