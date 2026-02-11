# css/css-grid/grid-model/grid-areas-overflowing-grid-container-007.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-model/grid-areas-overflowing-grid-container-007.html"
}
```

## style[0]

```css

    .grid {
       grid: 100px 100px / 100px 100px;
       width: 100px;
       height: 100px;
    }
    .item {
       grid-column: 2;
       grid-row: 2;
       width: 0px;
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
