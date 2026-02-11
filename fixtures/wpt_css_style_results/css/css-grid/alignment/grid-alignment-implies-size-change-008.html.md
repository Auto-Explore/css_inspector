# css/css-grid/alignment/grid-alignment-implies-size-change-008.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-alignment-implies-size-change-008.html"
}
```

## style[0]

```css

.grid {
  position: relative;
  display: inline-grid;
  grid-template-columns: 100px;
  grid-template-rows: 80px;
  font: 20px/1 Ahem;
  background: grey;
  align-items: stretch;
}
#item {
  background: blue;
  align-self: auto;
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
