# css/css-grid/grid-lanes/tentative/grid-placement/column-explicit-placement-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/column-explicit-placement-001-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    width: 240px;
    gap: 20px;
    padding: 20px;
    align-items: start;
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
