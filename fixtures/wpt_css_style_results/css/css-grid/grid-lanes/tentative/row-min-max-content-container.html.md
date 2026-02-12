# css/css-grid/grid-lanes/tentative/row-min-max-content-container.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/row-min-max-content-container.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:15px/1 monospace; padding:0; margin:0;
}

.grid-lanes {
    display: grid-lanes;
    background: gray;
    position: relative;
    flow-tolerance: 0;
    grid-lanes-direction: row;
    grid-template-rows: auto auto auto;
    padding: 10px;
}

.fourth-item {
  grid-row: span 2;
}

.invisible {
  visibility: hidden;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
