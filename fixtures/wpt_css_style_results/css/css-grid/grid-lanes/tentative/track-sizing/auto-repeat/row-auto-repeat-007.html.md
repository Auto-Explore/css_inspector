# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-007.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-007.html"
}
```

## style[0]

```css

.grid-lanes {
    position: relative;
    display: grid-lanes;
    grid-lanes-direction: row;
    grid-template-rows: repeat(auto-fill, 50px);
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
  /* Place item on the last row. */
  grid-row: -2;
  width: 300px;
  height: 100%;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
