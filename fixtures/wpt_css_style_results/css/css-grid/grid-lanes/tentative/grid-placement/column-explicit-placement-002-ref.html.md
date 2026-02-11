# css/css-grid/grid-lanes/tentative/grid-placement/column-explicit-placement-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/column-explicit-placement-002-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    width: 200px;
    gap: 20px;
    padding: 20px;
    align-items: start;
}

.first-track {
    background: lightskyblue;
    grid-column-start: 1;
    writing-mode: vertical-rl;
    margin-left: 10px;
}

.second-track {
    background: lightcoral;
    grid-column-start: 2;
}

.third-track {
    background: lightgreen;
    grid-column-start: 3;
    writing-mode: vertical-lr;
}
```

```json
{
  "errors": 3,
  "messages": [
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
