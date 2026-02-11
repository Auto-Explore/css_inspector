# css/css-grid/grid-lanes/tentative/items/row-minimum-size-grid-items-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/items/row-minimum-size-grid-items-001.html"
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

#constrained-grid-lanes {
    display: grid-lanes;
    grid-lanes-direction: row;
    width: 10px;
    height: 10px;
    grid-template-rows: minmax(auto, 0px);
}

#test-item-overlapping-red {
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
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
