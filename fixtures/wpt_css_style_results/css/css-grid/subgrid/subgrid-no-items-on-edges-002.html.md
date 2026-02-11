# css/css-grid/subgrid/subgrid-no-items-on-edges-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/subgrid-no-items-on-edges-002.html"
}
```

## style[0]

```css

.wrapper {
  width: 100px;
  height: 100px;
  background: red;
}
.grid {
  display: inline-grid;
  grid: repeat(2, auto) / repeat(4, auto);
  background: red;
}
.subgrid {
  display: grid;
  grid-template: subgrid / subgrid;
  background: green;
  padding: 11px 3px 9px 7px;
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
