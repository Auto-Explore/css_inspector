# css/css-grid/grid-items/grid-minimum-size-grid-items-015.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-minimum-size-grid-items-015.html"
}
```

## style[0]

```css

#reference-overlapped-red {
    position: absolute;
    background-color: red;
    width: 100px;
    height: 100px;
    z-index: -1;
}

#constrained-grid {
    display: grid;
    grid: 10px auto 10px / 10px auto 10px;
    justify-content: start;
}

#test-grid-item-overlapping-green {
    background-color: green;
    grid-row: span 3;
    grid-column: span 3;
}

#content-100x100 {
    width: 100px;
    height: 100px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
