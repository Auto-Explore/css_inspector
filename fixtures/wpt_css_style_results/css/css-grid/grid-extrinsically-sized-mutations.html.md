# css/css-grid/grid-extrinsically-sized-mutations.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-extrinsically-sized-mutations.html"
}
```

## style[0]

```css

.container {
  width: 50px;
}
.grid {
  display: grid;
  grid-template-columns: 100%;
  grid-template-rows: 50px;
  height: 50px;
  outline: 1px solid blue;
  font: 10px/1 Ahem;
}

.alignStart {
  align-items: start;
}

.fixedHeight {
  height: 100px;
}

.percentHeight {
  height: 100%;
}

.percentRow {
  grid-template-rows: 100%;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
