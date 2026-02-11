# css/css-grid/grid-lanes/tentative/grid-placement/row-explicit-placement-006-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/row-explicit-placement-006-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    height: 200px;
    gap: 20px;
    padding: 20px;
}

.first-track {
    background: lightskyblue;
    grid-row-start: 1;
    writing-mode: vertical-rl;
    margin-top: 10px;
    width: fit-content;
}

.second-track {
    background: lightcoral;
    grid-row-start: 2;
    width: fit-content;
}

.third-track {
    background: lightgreen;
    grid-row-start: 3;
    writing-mode: vertical-lr;
    width: fit-content;
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
