# css/css-grid/layout-algorithm/grid-content-distribution-must-account-for-track-sizing-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/grid-content-distribution-must-account-for-track-sizing-002.html"
}
```

## style[0]

```css

.grid {
  display: inline-grid;
  background: grey;
  grid-template-rows: 50px 50px;
  font: 20px/1 Ahem;
  height: 200px;
}

.item {
  grid-row: span 2;
  background: green;
  writing-mode: vertical-lr;
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
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
