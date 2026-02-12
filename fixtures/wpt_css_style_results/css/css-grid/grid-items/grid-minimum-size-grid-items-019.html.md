# css/css-grid/grid-items/grid-minimum-size-grid-items-019.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-minimum-size-grid-items-019.html"
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
    width: 500px;
    height: 200px;
    grid: 50% / 20%;
}

#test-grid-item-overlapping-green {
    background-color: green;
}

#content-200x200 {
    width: 200px;
    height: 200px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
