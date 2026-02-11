# css/css-grid/grid-lanes/tentative/items/column-flex-and-intrinsic-sizes.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/items/column-flex-and-intrinsic-sizes.html"
}
```

## style[0]

```css

.grid-lanes { display: grid-lanes; position: relative; grid-template-columns: repeat(12, 1fr); height: 100px; width: 100px; }
.test { grid-column: 1 / span 8; background: red; }
.over { grid-column: 1 / span 8; width: 100%; background: green; position: absolute; }
.under { grid-column: 9 / span 4; background: green; }
.big-child { width: 500px; height: 100px; }
.grid-lanes > div {
  height: 100px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
