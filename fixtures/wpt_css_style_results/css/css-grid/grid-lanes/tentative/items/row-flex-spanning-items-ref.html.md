# css/css-grid/grid-lanes/tentative/items/row-flex-spanning-items-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/items/row-flex-spanning-items-ref.html"
}
```

## style[0]

```css

#grid {
    display: grid;
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
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
