# css/css-grid/grid-lanes/tentative/items/row-flex-and-intrinsic-sizes.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/items/row-flex-and-intrinsic-sizes.html"
}
```

## style[0]

```css

.grid-lanes { display: grid-lanes; grid-lanes-direction: row; position: relative; grid-template-rows: repeat(12, 1fr); height: 100px; width: 100px; }
.test { grid-row: 1 / span 8; background: red; }
.over { grid-row: 1 / span 8; height: 100%; background: green; position: absolute; }
.under { grid-row: 9 / span 4; background: green; }
.big-child { height: 500px; width: 100px; }
.grid-lanes > div {
  width: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
