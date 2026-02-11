# css/css-grid/grid-lanes/tentative/intrinsic-sizing/column-intrinsic-inline-container-size.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/column-intrinsic-inline-container-size.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    background: gray;
    position: relative;
    flow-tolerance: 0;
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
}

.third-track {
    background: lightgreen;
    grid-column-start: 3;
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
