# css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/row-flow-tolerance-infinite-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/row-flow-tolerance-infinite-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    font: 15px/1 Ahem;
    grid-template-rows: repeat(4, 1fr);
    gap: 20px;
    padding: 20px;
    width: 380px;
}

.auto-item {
    padding: 10px;
    height: 50px;
    background: lightskyblue;
}

.flex {
  gap: 20px;
  display: flex;
  flex-direction: row;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
