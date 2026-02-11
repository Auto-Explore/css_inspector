# css/css-transforms/transform-matrix-009.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-matrix-009.html"
}
```

## style[0]

```css

.wrapper {
  width: 200px;
  height: 200px;
  background: red;
}
.test {
  width: 200px;
  height: 200px;
  background: green;
  transform: matrix(0,1, 1,0, 0,0);
  /*
    The resulting matrix is:
    ┌         ┐
    │ 0 1 0 0 │
    │ 1 0 0 0 │
    │ 0 0 1 0 │
    │ 0 0 0 1 │
    └         ┘
    It could result from e.g. `scaleX(-1) rotate(90deg)`.
  */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
