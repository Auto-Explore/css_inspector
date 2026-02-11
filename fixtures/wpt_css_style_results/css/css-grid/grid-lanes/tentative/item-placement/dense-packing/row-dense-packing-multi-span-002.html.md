# css/css-grid/grid-lanes/tentative/item-placement/dense-packing/row-dense-packing-multi-span-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/dense-packing/row-dense-packing-multi-span-002.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    flow-tolerance: 0;
    grid-lanes-direction: row;
    grid-template-rows: 10px 10px 20px 15px 5px;
    grid-auto-flow: dense;
    grid-lanes-pack: dense;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “grid-lanes-pack”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
