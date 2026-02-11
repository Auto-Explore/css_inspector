# css/css-grid/grid-lanes/tentative/grid-placement/row-explicit-placement-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/row-explicit-placement-001.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    grid-auto-flow: column;
    grid-lanes-direction: row;
    background: gray;
    flow-tolerance: 0;
    height: 200px;
    gap: 20px;
    padding: 20px;
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
```

```json
{
  "errors": 5,
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
    }
  ],
  "warnings": 0
}
```
