# css/css-grid/placement/grid-placement-using-named-grid-lines-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/placement/grid-placement-using-named-grid-lines-001.html"
}
```

## style[0]

```css

.grid {
    display: inline-grid;
    background: red;
    grid-template-columns: [C] 25px [C] 25px [C] 25px [C] 25px;
    grid-auto-rows: 50px;
    grid-template-areas: "A1 A2 A3 A4"
                         ".  A2 A3 A4";
}
.grid > div { background: green; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-template-areas”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
