# css/css-grid/grid-lanes/tentative/gap/column-gaps-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/gap/column-gaps-001.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    background: gray;
    position: relative;
    flow-tolerance: 0;
    grid-template-columns: repeat(3, 100px);
    width: 400px;
    padding: 20px;
    gap: 10px 50px;
}

.auto-item {
    padding: 10px;
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

.two-track-spanner {
    background: lightpink;
    grid-column: span 2;
    padding: 10px;
}
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “gap”.",
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
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
