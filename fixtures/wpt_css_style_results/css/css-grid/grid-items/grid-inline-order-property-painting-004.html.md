# css/css-grid/grid-items/grid-inline-order-property-painting-004.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-inline-order-property-painting-004.html"
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
    order: -5;
  }

  #reference-item-overlapped-red {
    color: red;
    order: -10;
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
