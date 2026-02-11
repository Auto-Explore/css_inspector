# css/css-grid/grid-lanes/tentative/item-placement/column-auto-placement-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/column-auto-placement-001-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    grid-template-columns: 100px 100px 100px;
    width: 340px;
    gap: 20px;
    padding: 20px;
    align-items: start;
}

.flex {
    display: flex;
    flex-direction: column;
    overflow: visible;
    flex-wrap: nowrap;
    gap: 20px;
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
    width: 200%;
    transform: translateX(-120px);
}
```

```json
{
  "errors": 4,
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
    }
  ],
  "warnings": 0
}
```
