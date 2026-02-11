# css/css-grid/grid-lanes/tentative/grid-placement/row-explicit-placement-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/row-explicit-placement-002.html"
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
    margin: 20px;
}

.second-track {
    background: lightcoral;
    grid-row-start: 2;
    height: 80px;
    margin-top: 40px;
    margin-bottom: 80px;
}

.third-track {
    background: lightgreen;
    grid-row-start: 3;
    margin-left: 40px;
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
