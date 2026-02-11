# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-mixed-intrinsic-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-mixed-intrinsic-002-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    flow-tolerance: 0;
    grid-template-columns: repeat(3, 135px 90px);
    grid-template-rows: 100px 100px 100px;
    width: 675px;
    height: 300px;
    font: 15px/1 Ahem;
}

.grid > div {
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
