# css/css-grid/grid-items/grid-z-axis-ordering-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-z-axis-ordering-001.html"
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
    z-index: 1;
  }

  #reference-item-overlapped-red {
    color: red;
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
