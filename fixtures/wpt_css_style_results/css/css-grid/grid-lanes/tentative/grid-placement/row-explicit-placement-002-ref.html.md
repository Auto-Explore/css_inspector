# css/css-grid/grid-lanes/tentative/grid-placement/row-explicit-placement-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/row-explicit-placement-002-ref.html"
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

.grid > div {
    width: fit-content;
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
