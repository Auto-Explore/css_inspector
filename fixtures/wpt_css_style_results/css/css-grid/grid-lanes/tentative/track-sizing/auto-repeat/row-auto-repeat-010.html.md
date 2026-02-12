# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-010.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-010.html"
}
```

## style[0]

```css

.grid-lanes {
    position: relative;
    display: grid-lanes;
    grid-lanes-direction: row;
    grid-template-rows: repeat(auto-fill, 50px);
    max-width: 50%;
    max-height: 80%;
    background: pink;
}
.wrapper {
  width: 600px;
  height: 250px;
}
.item {
  background: lime;
  /* Place item on the last row. */
  grid-row: -2;
  width: 300px;
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
