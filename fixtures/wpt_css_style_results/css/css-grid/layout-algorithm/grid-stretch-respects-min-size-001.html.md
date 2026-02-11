# css/css-grid/layout-algorithm/grid-stretch-respects-min-size-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/grid-stretch-respects-min-size-001.html"
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

  #constrained-wrapper {
    width: 50px;
    height: 50px;
  }

  #grid {
    display: grid;
    min-width: 100px;
    min-height: 100px;
  }

  #test-item-overlapping-green {
    background: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
