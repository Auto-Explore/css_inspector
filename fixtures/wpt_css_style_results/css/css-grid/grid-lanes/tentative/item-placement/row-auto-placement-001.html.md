# css/css-grid/grid-lanes/tentative/item-placement/row-auto-placement-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/row-auto-placement-001.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    background: gray;
    position: relative;
    flow-tolerance: 0;
    grid-lanes-direction: row;
    grid-template-rows: auto auto auto;
    width: 300px;
    padding: 10px;
}

.first-track {
    background: lightskyblue;
    grid-row-start: 1;
}

.second-track {
    background: lightcoral;
    grid-row-start: 2;
}

.third-track {
    background: lightgreen;
    grid-row-start: 3;
}

.square-100x100 {
    width: 100px;
    height: 100px;
}
```

```json
{
  "errors": 5,
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
