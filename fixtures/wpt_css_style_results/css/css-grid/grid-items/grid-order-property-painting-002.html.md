# css/css-grid/grid-items/grid-order-property-painting-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-order-property-painting-002.html"
}
```

## style[0]

```css

  #grid {
    display: grid;
    font: 100px/1 Ahem;
  }

  #test-item-overlapping-green {
    color: green;
  }

  #reference-item-overlapped-red {
    color: red;
    order: -1;
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
