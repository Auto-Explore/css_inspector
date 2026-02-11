# css/css-grid/grid-items/grid-minimum-size-grid-items-013.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-minimum-size-grid-items-013.html"
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
    grid: 10px / 10px;
}

#test-grid-item-overlapping-green {
    color: green;
    background-color: green;
    font: 50px/1 Ahem;
    justify-self: start;
    align-self: start;
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
