# css/css-grid/grid-items/grid-minimum-size-grid-items-017.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-minimum-size-grid-items-017.html"
}
```

## style[0]

```css

#reference-overlapped-green {
    position: absolute;
    background-color: green;
    width: 100px;
    height: 100px;
    z-index: -1;
}

#constrained-grid {
    display: grid;
    width: 10px;
    height: 10px;
    grid: minmax(auto, 0px) / minmax(auto, 0px);
}

#test-grid-item-overlapping-red {
    background-color: red;
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
