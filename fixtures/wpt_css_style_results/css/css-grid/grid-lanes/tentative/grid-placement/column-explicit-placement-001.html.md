# css/css-grid/grid-lanes/tentative/grid-placement/column-explicit-placement-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/column-explicit-placement-001.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    background: gray;
    position: relative;
    flow-tolerance: 0;
    width: 240px;
    gap: 20px;
    padding: 20px;
}

.first-track {
    background: lightskyblue;
    grid-column-start: 1;
}

.second-track {
    background: lightcoral;
    grid-column-start: 2;
    margin-left: 40px;
}

.third-track {
    background: lightgreen;
    grid-column-start: 3;
    margin-top: 40px;
    margin-right: 40px;
}

.fourth-track {
    background: purple;
    grid-column-start: 4;
}
```

```json
{
  "errors": 4,
  "messages": [
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
