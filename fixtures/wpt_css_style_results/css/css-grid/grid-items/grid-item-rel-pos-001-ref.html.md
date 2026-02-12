# css/css-grid/grid-items/grid-item-rel-pos-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-item-rel-pos-001-ref.html"
}
```

## style[0]

```css

.grid {
  margin: 40px;
  display: grid;
  grid: auto / repeat(3,100px);
  grid-gap: 20px;
}
span, even {
  position: relative;
  min-height: 20px;
  background: grey;
  left: 0px;
}
.offset even {
  left: 50%;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
