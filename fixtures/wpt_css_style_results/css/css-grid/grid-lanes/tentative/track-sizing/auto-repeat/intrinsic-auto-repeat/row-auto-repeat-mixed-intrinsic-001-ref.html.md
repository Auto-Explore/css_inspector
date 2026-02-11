# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-mixed-intrinsic-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-mixed-intrinsic-001-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    flow-tolerance: 0;
    grid-template-rows: min-content max-content fit-content(100px) auto;
    grid-template-columns: 100px 100px;
    height: 500px;
    width: 300px;
    font: 15px/1 Ahem;
}

.grid > div {
    width: 100px;
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
