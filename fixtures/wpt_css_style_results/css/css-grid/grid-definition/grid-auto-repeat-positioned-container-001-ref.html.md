# css/css-grid/grid-definition/grid-auto-repeat-positioned-container-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/grid-auto-repeat-positioned-container-001-ref.html"
}
```

## style[0]

```css


.grid {
    width: 100px;
    grid: repeat(5, 20px) / repeat(4, 25px);
    justify-content: start;
    align-content: start;
}

.item {
    background: green;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
