# css/css-grid/grid-items/grid-z-axis-ordering-005.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-z-axis-ordering-005.html"
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
    z-index: -1;
  }

  .first-row-first-column {
    grid-area: 1 / 1;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
