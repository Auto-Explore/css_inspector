# css/css-grid/grid-lanes/tentative/intrinsic-sizing/row-intrinsic-inline-container-size.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/row-intrinsic-inline-container-size.html"
}
```

## style[0]

```css

.grid-lanes {
    grid-lanes-direction: row;
    grid-template-rows: repeat(3, 20px);
    display: grid-lanes;
    background: gray;
    position: relative;
    flow-tolerance: 0;
}

.max-content-item {
    width: max-content;
    height: 20px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
