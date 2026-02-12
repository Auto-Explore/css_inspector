# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-008.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-008.html"
}
```

## style[0]

```css

.grid-lanes {
    position: relative;
    display: grid-lanes;
    grid-template-columns: repeat(auto-fill, 100px);
    min-width: 300px;
    min-height: 200px;
    float: left;
    background: pink;
}
.border {
  border: 10px solid;
}
.border-box {
  box-sizing: border-box;
}
.item {
  background: lime;
  /* Place item on the last column. */
  grid-column: -2;
  width: 100%;
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
