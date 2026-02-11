# css/css-grid/grid-items/grid-z-axis-ordering-overlapped-items-004.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-z-axis-ordering-overlapped-items-004.html"
}
```

## style[0]

```css

  #grid {
    display: grid;
    font: 100px/1 Ahem;
    grid-template-columns: 25px 25px 25px 25px;
    grid-template-rows: 25px 25px 25px 25px;
  }

  #blue {
    color: blue;
    grid-column: 1;
    grid-row: 1;
  }

  #yellow {
    color: yellow;
    z-index: 1;
    grid-column: 2;
    grid-row: 2;
  }

  #green {
    color: green;
    z-index: 5;
    grid-column: 3;
    grid-row: 3;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
