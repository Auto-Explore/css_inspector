# css/css-grid/grid-lanes/tentative/gap/row-gaps-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/gap/row-gaps-001-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    grid-template-rows: repeat(3, 100px);
    height: 400px;
    width: 380px;
    padding: 20px;
    gap: 50px 10px;
}

.grid > div {
    width: 100px;
    grid-column: 1;
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
    grid-row: 1 / span 2;
    padding: 10px;
}
```

```json
{
  "errors": 6,
  "messages": [
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
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
