# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-mixed-intrinsic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-mixed-intrinsic-001.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    background: gray;
    flow-tolerance: 0;
    grid-lanes-direction: row;
    grid-template-rows: repeat(auto-fill, min-content max-content fit-content(100px) auto);
    height: 500px;
    width: 300px;
    font: 15px/1 Ahem;
}

.grid-lanes > div {
    width: 100px;
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
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
