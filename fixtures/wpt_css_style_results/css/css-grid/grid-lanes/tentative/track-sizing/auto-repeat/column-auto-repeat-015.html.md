# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-015.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-015.html"
}
```

## style[0]

```css

.grid-lanes {
  display: inline-grid-lanes;
  border: 1px solid black;
  grid-template-columns: [u] repeat(auto-fill, [v] 10px [w] 10px [x] 10px [y]) [z];
  grid-column-gap: 3px;
  /* Does not fit a whole-number of repetitions */
  width: 94px;
  background: pink;
  vertical-align: top;
}
div > div {
  background: blue;
  width: 100%;
  height: 25px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
