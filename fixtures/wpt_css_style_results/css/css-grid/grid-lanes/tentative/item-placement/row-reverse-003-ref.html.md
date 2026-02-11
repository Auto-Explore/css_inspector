# css/css-grid/grid-lanes/tentative/item-placement/row-reverse-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/row-reverse-003-ref.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    flow-tolerance: 0;
    grid-template-rows: repeat(3, 50px);
    grid-lanes-direction: row;
    direction: rtl;
    gap: 10px;
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
