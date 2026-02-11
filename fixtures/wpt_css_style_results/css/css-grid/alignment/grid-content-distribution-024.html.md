# css/css-grid/alignment/grid-content-distribution-024.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-content-distribution-024.html"
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
  width: 50px;
  height: 50px;
}
.grid {
  z-index: 1;
  display: grid;
  width: 200px;
  height: 200px;
  grid-template-columns: auto auto auto auto;
  grid-template-rows: auto auto auto auto;
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
