# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-row-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-row-001-ref.html"
}
```

## style[0]

```css

#grid {
  border: 3px solid black;
  display: grid;
  grid-template-rows: 55px 66px;
  grid-template-columns: repeat(2, auto);
  contain-intrinsic-size: 70px 80px;
  contain: size;
  width: max-content;
  background: lightblue;
  grid-gap: 5px;
}
.item {
  background: lightgreen;
  opacity: 0.5;
  width: 50px;
  height: 50px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
