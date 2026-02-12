# css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/row-initial-flow-tolerance-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/row-initial-flow-tolerance-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    font: 50px/1 Ahem;
    grid-template-rows: repeat(2, 1fr);
    gap: 20px;
    padding: 20px;
    width: 170px;
}

.auto-item {
    padding: 10px;
    height: 100px;
}

.auto-item:nth-of-type(1) {
    background: lightskyblue;
    grid-column: 1;
}

.auto-item:nth-of-type(2) {
    background: lightcoral;
    grid-column: 1;
}

.auto-item:nth-of-type(3) {
    background: palegoldenrod;
    grid-column: 2;
    transform: translateX(-50px);
}

.auto-item:nth-of-type(4) {
    background: lightgreen;
    grid-column: 2;
    grid-row: 1;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
