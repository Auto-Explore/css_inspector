# css/css-grid/grid-lanes/tentative/items/column-minimum-size-grid-items-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/items/column-minimum-size-grid-items-002.html"
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
  "errors": 2,
  "messages": [
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
