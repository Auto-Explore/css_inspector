# css/css-grid/grid-lanes/tentative/items/row-flex-spanning-items.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/items/row-flex-spanning-items.html"
}
```

## style[0]

```css

#grid-lanes {
    display: grid-lanes;
    grid-lanes-direction: row;
    grid-template-rows: 1fr 30px;
    border: 10px solid fuchsia;
    height: min-content;
}
#item {
    grid-row: 1 / span 2;
}
#filler {
    height: 300px;
    width: 50px;
    background: aqua;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
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
