# css/css-grid/grid-lanes/tentative/items/row-minimum-contribution-baseline-shim.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/items/row-minimum-contribution-baseline-shim.html"
}
```

## style[0]

```css

.grid-lanes {
  display: grid-lanes;
  grid-lanes-direction: row;
  position: relative;
  font-size: 0;
  height: 0;
  width: 0;
  margin-bottom: 125px;
  align-items: baseline;
}
.item1 {
  padding-top: 25px;
  background: yellow;
}
.item2 {
  padding-bottom: 25px;
  background: magenta;
}
.item1::before, .item2::before {
  content: '';
  display: inline-block;
  width: 25px;
  height: 25px;
  vertical-align: bottom;
}
.item2::before {
  vertical-align: top;
}
.area {
  position: absolute;
  z-index: -1;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
  grid-column: 1 / 3;
  grid-row: 1 / 2;
  background: cyan;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
