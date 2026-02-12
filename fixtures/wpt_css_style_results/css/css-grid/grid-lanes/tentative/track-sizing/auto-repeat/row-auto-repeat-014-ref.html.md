# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-014-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-014-ref.html"
}
```

## style[0]

```css

.grid {
  display: inline-grid;
  border: 1px solid black;
  grid-template-rows: [u] repeat(auto-fill, [v] 10px [w] 10px [x] 10px [y]) [z];
  grid-row-gap: 3px;
  width: min-content;
  /* Does not fit a whole-number of repetitions */
  height: 94px;
  background: pink;
}
div > div {
  background: blue;
  width: 25px;
  height: 100%;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
