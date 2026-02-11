# css/css-grid/grid-lanes/tentative/items/column-flex-spanning-items.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/items/column-flex-spanning-items.html"
}
```

## style[0]

```css

#grid-lanes {
    display: grid-lanes;
    grid-template-columns: 1fr 30px;
    border: 10px solid fuchsia;
    width: min-content;
}
#item {
    grid-column: 1 / span 2;
}
#filler {
    width: 300px;
    height: 50px;
    background: aqua;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
