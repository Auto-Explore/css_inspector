# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-auto-029.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-auto-029.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    background: gray;
    flow-tolerance: 0;
    grid-template-rows: repeat(auto-fill, 100px auto);
    height: 500px;
    width: 300px;
}

.grid-lanes > div {
    height: 300px;
    width: 100px;
    grid-row: span 2;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
