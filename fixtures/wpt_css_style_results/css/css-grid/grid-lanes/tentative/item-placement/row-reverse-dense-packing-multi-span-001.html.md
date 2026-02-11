# css/css-grid/grid-lanes/tentative/item-placement/row-reverse-dense-packing-multi-span-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/row-reverse-dense-packing-multi-span-001.html"
}
```

## style[0]

```css

.grid-lanes {
  display: grid-lanes;
  flow-tolerance: 0;
  grid-auto-flow: dense;
  grid-lanes-pack: dense;
  grid-lanes-direction: row track-reverse;
  grid-template-rows: repeat(4, 50px);
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
