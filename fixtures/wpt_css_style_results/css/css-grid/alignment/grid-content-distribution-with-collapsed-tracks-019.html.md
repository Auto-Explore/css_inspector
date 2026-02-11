# css/css-grid/alignment/grid-content-distribution-with-collapsed-tracks-019.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-content-distribution-with-collapsed-tracks-019.html"
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
  grid-template-columns: repeat(auto-fit, 20px);
  grid-template-rows: repeat(auto-fit, 20px);
  grid-row-gap: 20px;
  grid-column-gap: 20px;
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
