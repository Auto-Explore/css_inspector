# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-auto-009.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-auto-009.html"
}
```

## style[0]

```css

.grid-lanes {
    position: relative;
    display: grid-lanes;
    grid-lanes-direction: row;
    grid-template-rows: repeat(auto-fill, auto);
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
  /* Place item on the last row. */
  grid-row: -2;
  width: 300px;
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
