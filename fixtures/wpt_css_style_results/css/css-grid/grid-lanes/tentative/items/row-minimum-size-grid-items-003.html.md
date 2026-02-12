# css/css-grid/grid-lanes/tentative/items/row-minimum-size-grid-items-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/items/row-minimum-size-grid-items-003.html"
}
```

## style[0]

```css

.grid-lanes {
  display: grid-lanes;
  grid-lanes-direction: row;
  border: solid thick;
  font: 10px/1 Ahem;
  width: 50px;
  height: 50px;
  margin: 50px 0px;
}

.grid-lanes > div {
  grid-row: span 2;
}

.grid-lanes > div:nth-child(1) {
  color: blue;
  background: cyan;
}

.grid-lanes > div:nth-child(2) {
  background: magenta;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
