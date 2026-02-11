# css/css-grid/alignment/grid-content-distribution-017.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-content-distribution-017.html"
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
  width: 20px;
  height: 20px;
}
.grid {
  z-index: 1;
  display: grid;
  width: 300px;
  height: 300px;
  grid-template-columns: 20px 20px 20px;
  grid-template-rows: 20px 20px 20px;
  align-content: space-around;
  justify-content: space-around;
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
