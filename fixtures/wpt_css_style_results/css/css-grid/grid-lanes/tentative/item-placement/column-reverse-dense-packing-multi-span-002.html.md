# css/css-grid/grid-lanes/tentative/item-placement/column-reverse-dense-packing-multi-span-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/column-reverse-dense-packing-multi-span-002.html"
}
```

## style[0]

```css

.grid-lanes {
  display: grid-lanes;
  flow-tolerance: 0;
  grid-auto-flow: dense;
  grid-lanes-pack: dense;
  grid-lanes-direction: column track-reverse;
  grid-template-columns: 10px 10px 20px 15px 5px;
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
      "message": "Unknown property “grid-lanes-pack”.",
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
