# css/css-grid/subgrid/standalone-axis-size-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/standalone-axis-size-002.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  grid-template-rows: 100px;
}
.subgrid {
  display: grid;
  width: min-content;
  grid-template: subgrid / 20% 30%;
}
.w100 { width: 100px }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
