# css/css-grid/grid-items/grid-minimum-size-grid-items-011.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-minimum-size-grid-items-011.html"
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
    width: 100px;
    height: 100px;
    grid: minmax(0px, 500px) / minmax(0px, 500px);
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
