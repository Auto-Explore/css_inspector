# css/css-grid/grid-lanes/tentative/gap/row-gaps-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/gap/row-gaps-001.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    grid-lanes-direction: row;
    background: gray;
    flow-tolerance: 0;
    grid-template-rows: repeat(3, 100px);
    height: 400px;
    width: 380px;
    padding: 20px;
    gap: 50px 10px;
}

.grid-lanes > div {
    width: 100px;
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
    grid-row: span 2;
    padding: 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
