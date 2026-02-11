# css/css-grid/grid-model/grid-areas-overflowing-grid-container-004.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-model/grid-areas-overflowing-grid-container-004.html"
}
```

## style[0]

```css

    .grid {
       grid: 100px / 100%;
       width: 100px;
       height: 100px;
       overflow: auto;
    }
    .item {
       grid-column: 1;
       grid-row: 1;
       margin-top: 100px;
       width: 50px;
       height: 50px;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
