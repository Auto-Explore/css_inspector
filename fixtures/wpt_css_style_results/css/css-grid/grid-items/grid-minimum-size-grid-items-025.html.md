# css/css-grid/grid-items/grid-minimum-size-grid-items-025.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-minimum-size-grid-items-025.html"
}
```

## style[0]

```css

.grid {
  border: solid thick;
  font: 10px/1 Ahem;
  width: 50px;
  height: 50px;
  grid-template-columns: 25px 25px;
  margin: 50px 0px;
}

.grid > div {
  grid-row: span 2;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
