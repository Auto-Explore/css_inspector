# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-012.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-012.html"
}
```

## style[0]

```css

.grid-lanes {
    position: relative;
    display: grid-lanes;
    grid-template-columns: repeat(auto-fill, 100px);
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
  width: 100%;
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
