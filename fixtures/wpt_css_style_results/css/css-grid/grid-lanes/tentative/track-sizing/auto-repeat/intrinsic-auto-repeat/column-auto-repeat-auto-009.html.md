# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-009.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-009.html"
}
```

## style[0]

```css

.grid-lanes {
    position: relative;
    display: grid-lanes;
    grid-template-columns: repeat(auto-fill, auto);
    min-width: 300px;
    min-height: 200px;
    float: left;
    background: pink;
}
.border {
  border: 10px solid;
}
.item {
  background: lime;
  /* Place item on the last column. */
  grid-column: -2;
  width: 150px;
  height: 200px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
