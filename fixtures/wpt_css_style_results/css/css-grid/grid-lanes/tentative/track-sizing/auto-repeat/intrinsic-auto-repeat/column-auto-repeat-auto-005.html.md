# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-005.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-005.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    background: gray;
    flow-tolerance: 0;
    /*
      This is not currently a valid track definition and will fall back to
      none.
    */
    grid-template-columns: auto repeat(auto-fill, auto) auto;
    width: 300px;
    height: 300px;
}

.grid-lanes > div {
    width: 100px;
    height: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
