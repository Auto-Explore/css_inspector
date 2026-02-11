# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-011.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-011.html"
}
```

## style[0]

```css

.grid-lanes {
    position: relative;
    display: grid-lanes;
    grid-template-rows: repeat(auto-fill, 50px);
    grid-lanes-direction: row;
    max-width: 100px;
    min-width: 250px;
    max-height: 50px;
    min-height: 125px;
    float: left;
    background: pink;
}
.item {
  background: lime;
  /* Place item on the last row. */
  grid-row: -2;
  width: 250px;
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
