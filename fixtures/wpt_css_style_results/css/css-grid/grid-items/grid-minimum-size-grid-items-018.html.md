# css/css-grid/grid-items/grid-minimum-size-grid-items-018.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-minimum-size-grid-items-018.html"
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
    width: 10px;
    height: 10px;
    grid: minmax(auto, 500px) / minmax(auto, 500px);
}

#test-grid-item-overlapping-green {
    background-color: green;
}

#content-100x100 {
    width: 100px;
    height: 100px;
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
