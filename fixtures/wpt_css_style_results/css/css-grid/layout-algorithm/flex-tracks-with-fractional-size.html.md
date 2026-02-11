# css/css-grid/layout-algorithm/flex-tracks-with-fractional-size.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/flex-tracks-with-fractional-size.html"
}
```

## style[0]

```css

.grid {
  width: 100px;
  height: 50px;
  display: grid;
  background: red;
}
.grid > div {
  background: green;
}
#cols {
  grid-template-rows: auto 0px;
  grid-template-columns: repeat(973, 1fr);
}
#rows {
  grid-auto-flow: column;
  grid-template-columns: auto 0px;
  grid-template-rows: repeat(973, 1fr);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
