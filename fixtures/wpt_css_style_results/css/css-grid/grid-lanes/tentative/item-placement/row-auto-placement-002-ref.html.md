# css/css-grid/grid-lanes/tentative/item-placement/row-auto-placement-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/row-auto-placement-002-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    grid-template-rows: auto auto auto;
    grid-template-columns: auto auto auto;
    width: 870px;
    gap: 20px;
    padding: 20px;
    align-items: start;
    font: 15px/1 "Ahem";
}

.grid > div {
    width: max-content;
}

.grid-auto-item-1 {
    background: lightskyblue;
    padding: 10px;
}

.grid-auto-item-2 {
    background: lightcoral;
    padding: 10px;
}

.grid-auto-item-3 {
    background: lightgreen;
    padding: 10px;
}

.grid-auto-item-4 {
    background: lightpink;
    padding: 10px;
    grid-row: 1 / 3;
    align-self: stretch;
    transform: translateX(-3em);
}
```

```json
{
  "errors": 5,
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
