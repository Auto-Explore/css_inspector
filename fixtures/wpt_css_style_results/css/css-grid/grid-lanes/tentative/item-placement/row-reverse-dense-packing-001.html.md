# css/css-grid/grid-lanes/tentative/item-placement/row-reverse-dense-packing-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/row-reverse-dense-packing-001.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    flow-tolerance: 0;
    grid-template-rows: repeat(4, 50px) 15px;
    gap: 10px;
    grid-lanes-direction: row track-reverse;
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
