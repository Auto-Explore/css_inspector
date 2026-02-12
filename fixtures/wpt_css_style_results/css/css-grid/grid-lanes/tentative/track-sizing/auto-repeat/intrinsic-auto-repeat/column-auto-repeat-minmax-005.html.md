# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-minmax-005.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-minmax-005.html"
}
```

## style[0]

```css

.grid-lanes {
  display: grid-lanes;
  border: solid thick;
  margin: 10px;
  grid-template-columns: repeat(auto-fill, minmax(auto, auto) minmax(auto, auto));
  grid-column-gap: 100px;
  width: 300px;
  background: pink;
}
.grid-lanes > div {
  background: lime;
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
