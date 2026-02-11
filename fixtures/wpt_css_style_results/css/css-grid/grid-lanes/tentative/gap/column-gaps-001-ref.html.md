# css/css-grid/grid-lanes/tentative/gap/column-gaps-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/gap/column-gaps-001-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    grid-template-columns: repeat(3, 100px);
    width: 400px;
    gap: 50px;
    padding: 20px;
    align-items: start;
}

.flex {
    display: flex;
    flex-direction: column;
    overflow: visible;
    flex-wrap: nowrap;
    gap: 10px;
}

.grid-auto-item-1 {
    background: lightskyblue;
    width: 80px;
    padding: 10px;
}

.grid-auto-item-2 {
    background: lightcoral;
    width: 80px;
    padding: 10px;
}

.grid-auto-item-3 {
    background: lightgreen;
    width: 80px;
    padding: 10px;
}

.grid-auto-item-4 {
    background: lightpink;
    padding: 10px;
    width: calc(200% + 30px);
    transform: translateX(-150px);
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
