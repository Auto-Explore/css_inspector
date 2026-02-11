# css/css-grid/abspos/grid-item-absolute-positioning-dynamic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/grid-item-absolute-positioning-dynamic-001.html"
}
```

## style[0]

```css

#wrapper {
  width: 200px;
  height: 200px;
  position: relative;
  background: red;
}

#grid {
  display: grid;
  grid: 100px / 100px;
}

#item {
  background: green;
  width: 100%;
  height: 100%;
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
