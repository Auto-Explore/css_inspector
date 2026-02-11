# css/css-grid/alignment/grid-alignment-implies-size-change-027.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-alignment-implies-size-change-027.html"
}
```

## style[0]

```css

.grid {
  position: relative;
  display: inline-grid;
  grid-template-columns: 80px;
  grid-template-rows: 100px;
  font: 20px/1 Ahem;
  background: grey;
  justify-items: start;
}
#item {
  background: blue;
  justify-self: auto;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
