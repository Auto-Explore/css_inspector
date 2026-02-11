# css/css-grid/grid-items/grid-minimum-size-grid-items-020.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-minimum-size-grid-items-020.html"
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
    width: 200px;
    height: 500px;
    grid: 10% 30px 20px / 50px 30px 10%;
}

#test-grid-item-overlapping-green {
    background-color: green;
    grid-row: span 3;
    grid-column: span 3;
}

#content-200x200 {
    width: 200px;
    height: 200px;
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
