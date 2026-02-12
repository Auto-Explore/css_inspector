# css/css-grid/grid-lanes/tentative/grid-placement/row-explicit-placement-007.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/grid-placement/row-explicit-placement-007.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    grid-auto-flow: column;
    grid-lanes-direction: row;
    background: gray;
    flow-tolerance: 0;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
