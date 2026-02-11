# css/css-grid/grid-definition/grid-auto-repeat-positioned-container-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/grid-auto-repeat-positioned-container-001.html"
}
```

## style[0]

```css

.wrapper {
    position: relative;
    width: 100px;
    height: 100px;
}

.grid {
    position: absolute;
    left: 0; right: 0; top: 0; bottom: 0;
    grid: repeat(auto-fill, 20px) / repeat(auto-fill, 25px);
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
