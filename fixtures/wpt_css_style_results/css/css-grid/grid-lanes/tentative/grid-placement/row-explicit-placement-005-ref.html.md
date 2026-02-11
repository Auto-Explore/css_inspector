# css/css-grid/grid-lanes/tentative/grid-placement/row-explicit-placement-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/row-explicit-placement-005-ref.html"
}
```

## style[0]

```css

#grid {
    display: grid;
    grid-template-columns: 200px 200px;
    background: gray;
    height: 200px;
    width: 200px;
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
    height: 200%;
}

.third-track {
    background: lightgreen;
    grid-row-start: 3;
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
