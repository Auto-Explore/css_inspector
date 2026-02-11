# css/css-grid/grid-items/grid-minimum-size-grid-items-024.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-minimum-size-grid-items-024.html"
}
```

## style[0]

```css

.grid {
  border: solid thick;
  font: 10px/1 Ahem;
  width: 50px;
  height: 50px;
  grid-template-rows: 25px 25px;
}

.grid > div {
  grid-column: span 2;
}

.grid > div:nth-child(1) {
  color: blue;
  background: cyan;
}

.grid > div:nth-child(2) {
  background: magenta;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
