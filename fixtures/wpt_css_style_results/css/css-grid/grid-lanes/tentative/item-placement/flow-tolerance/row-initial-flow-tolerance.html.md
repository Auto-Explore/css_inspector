# css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/row-initial-flow-tolerance.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/row-initial-flow-tolerance.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    grid-lanes-direction: row;
    background: gray;
    font: 50px/1 Ahem;
    grid-template-rows: repeat(2, 1fr);
    flow-tolerance: initial;
    gap: 20px;
    padding: 20px;
    width: 170px;
}

.auto-item {
    padding: 10px;
    height: 100px;
}

.auto-item:nth-of-type(1) {
    background: lightskyblue;
}

.auto-item:nth-of-type(2) {
    background: lightcoral;
}

.auto-item:nth-of-type(3) {
    background: lightgreen;
}

.auto-item:nth-of-type(4) {
    background: palegoldenrod;
}
```

```json
{
  "errors": 6,
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
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
