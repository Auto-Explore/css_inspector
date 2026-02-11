# css/css-grid/abspos/grid-positioned-item-dynamic-change-007.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/grid-positioned-item-dynamic-change-007.html"
}
```

## style[0]

```css

.grid {
  display: inline-grid;
  grid-gap: 8px;
  position: relative;
}

.green {
  background: green;
  width: 100px;
  height: 100px;
}

.blue {
  position: absolute;
  border: 3px solid blue;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  box-sizing: border-box;
  grid-column: 1 / 2;
  grid-row: 2 / 3;
}

```

```json
{
  "errors": 2,
  "messages": [
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
