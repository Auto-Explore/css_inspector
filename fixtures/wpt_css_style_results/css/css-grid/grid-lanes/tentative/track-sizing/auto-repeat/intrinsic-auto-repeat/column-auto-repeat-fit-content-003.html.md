# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-fit-content-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-fit-content-003.html"
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
    grid-template-columns: fit-content(100px) repeat(auto-fill, fit-content(100px)) fit-content(100px);
    width: 300px;
    height: 300px;
    font: 15px/1 Ahem;
}

.grid-lanes > div {
    height: 100px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
