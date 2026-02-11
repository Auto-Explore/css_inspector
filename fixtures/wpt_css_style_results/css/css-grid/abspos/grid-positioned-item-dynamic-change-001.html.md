# css/css-grid/abspos/grid-positioned-item-dynamic-change-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/grid-positioned-item-dynamic-change-001.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  grid: 50px 50px / 50px 50px;
  position: relative;
}

.green {
  background: green;
}

.red {
  background: red;
}

#item {
  position: absolute;
  width: 100%;
  height: 100%;
  grid-column: 1 / 2;
  grid-row: 1 / 2;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
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
