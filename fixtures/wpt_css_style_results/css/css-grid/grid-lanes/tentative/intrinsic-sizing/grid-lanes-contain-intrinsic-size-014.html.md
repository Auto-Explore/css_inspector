# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-014.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-014.html"
}
```

## style[0]

```css

.grid-lanes {
  display: grid-lanes;
  border: 1px solid black;
  grid-template-columns: repeat(auto-fit, 100px);
  height: 40px;
}
.one {
  contain-intrinsic-size: 100px;
  contain: size;
  min-width: 200px;
}
.two {
  contain-intrinsic-size: 200px;
  contain: size;
  min-width: 100px;
}
.three {
  contain-intrinsic-size: 100px;
  contain: size;
  min-width: 200px;
  max-width: 150px;
}
.four {
  contain-intrinsic-size: 200px;
  contain: size;
  min-width: 100px;
  max-width: 150px;
}
.item {
  background: green;
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
