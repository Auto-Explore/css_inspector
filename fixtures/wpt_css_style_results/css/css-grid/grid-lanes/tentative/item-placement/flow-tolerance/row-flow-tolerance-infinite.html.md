# css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/row-flow-tolerance-infinite.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/row-flow-tolerance-infinite.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    grid-lanes-direction: row;
    background: gray;
    grid-template-rows: repeat(4, 1fr);
    flow-tolerance: infinite;
    gap: 20px;
    padding: 20px;
    width: 380px;
    font: 15px/1 Ahem;
}

.auto-item {
    padding: 10px;
    height: 50px;
    background: lightskyblue;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
