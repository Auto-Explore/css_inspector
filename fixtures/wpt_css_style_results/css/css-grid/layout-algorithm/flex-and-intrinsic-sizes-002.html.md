# css/css-grid/layout-algorithm/flex-and-intrinsic-sizes-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/flex-and-intrinsic-sizes-002.html"
}
```

## style[0]

```css

.grid { display: grid; grid-template-columns: repeat(12, 1fr); height: 100px; width: 100px; }
.test { grid-column: 1 / span 8; grid-row: 1; background: red; }
.over { grid-column: 1 / span 8; grid-row: 1; background: green; }
.under { grid-column: 9 / span 4; grid-row: 1; background: green; }
.big-child { width: 500px; height: 100px; }
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
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
