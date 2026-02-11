# css/css-grid/grid-items/grid-inline-z-axis-ordering-004.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-inline-z-axis-ordering-004.html"
}
```

## style[0]

```css

  #inline-grid {
    display: inline-grid;
    font: 100px/1 Ahem;
  }

  #test-item-overlapping-green {
    color: green;
    z-index: -5;
  }

  #reference-item-overlapped-red {
    color: red;
    z-index: -10;
  }

  .first-row-first-column {
    grid-area: 1 / 1;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-area”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
