# css/css-grid/grid-lanes/tentative/items/column-minimum-size-grid-items-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/items/column-minimum-size-grid-items-003.html"
}
```

## style[0]

```css

.grid-lanes {
  display: grid-lanes;
  border: solid thick;
  font: 10px/1 Ahem;
  width: 50px;
  height: 50px;
  grid-template-rows: 25px 25px;
}

.grid-lanes > div {
  grid-column: span 2;
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
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
