# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-009.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-009.html"
}
```

## style[0]

```css

.grid-lanes {
    position: relative;
    display: grid-lanes;
    grid-template-columns: repeat(auto-fill, 100px);
    min-width: 50%;
    min-height: 80%;
    float: left;
    background: pink;
}
.wrapper {
  width: 600px;
  height: 250px;
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
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
