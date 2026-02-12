# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-010.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-010.html"
}
```

## style[0]

```css

.grid-lanes {
    position: relative;
    display: grid-lanes;
    grid-template-columns: repeat(auto-fill, auto);
    max-width: 100px;
    min-width: 250px;
    max-height: 50px;
    min-height: 125px;
    float: left;
    background: pink;
}
.item {
  background: lime;
  /* Place item on the last column. */
  grid-column: -2;
  width: 100px;
  height: 125px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
