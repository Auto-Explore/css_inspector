# css/css-grid/grid-lanes/tentative/item-placement/dense-packing/column-dense-packing-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/dense-packing/column-dense-packing-005-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    position: relative;
    flow-tolerance: 0;
    grid-template-columns: repeat(3, 50px);
    grid-template-rows: grid-lanes;
    grid-auto-flow: dense;
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
      "message": "Invalid value for property “grid-template-rows”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
