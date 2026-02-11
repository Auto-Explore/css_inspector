# css/css-grid/grid-items/grid-inline-order-property-painting-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-inline-order-property-painting-003.html"
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
    order: 10;
  }

  #reference-item-overlapped-red {
    color: red;
    order: 5;
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
