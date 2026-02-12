# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-row-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-row-004-ref.html"
}
```

## style[0]

```css

#grid {
  border: 3px solid black;
  display: grid;
  grid-template-rows: repeat(2, 35px);
  grid-template-columns: repeat(2, min-content);
  contain: size;
  contain-intrinsic-inline-size: 6px;
  width: max-content;
  background: lightblue;
  grid-gap: 5px;
}
.item {
  background: lightgreen;
  opacity: 0.5;
  width: 20px;
  height: 20px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
