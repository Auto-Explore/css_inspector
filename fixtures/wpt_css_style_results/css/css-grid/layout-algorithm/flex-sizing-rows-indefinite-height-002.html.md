# css/css-grid/layout-algorithm/flex-sizing-rows-indefinite-height-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/flex-sizing-rows-indefinite-height-002.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  width: 100px;
  background: red;
}
.item {
  grid-row: 2 / span 1;
  background: green;
  width: 100px;
  height: 100px;
  position: relative;
  top: -32px; /* To move this item up to cover the first row */
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
