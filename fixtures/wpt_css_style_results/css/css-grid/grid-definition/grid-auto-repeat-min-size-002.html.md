# css/css-grid/grid-definition/grid-auto-repeat-min-size-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/grid-auto-repeat-min-size-002.html"
}
```

## style[0]

```css

.grid {
  position: relative;
  display: grid;
  grid: repeat(auto-fill, 50px) / repeat(auto-fill, 100px);
  min-width: 50%;
  min-height: 80%;
  float: left;
}
.wrapper {
  width: 600px;
  height: 250px;
}
.item {
  background: lime;
  /* Place item on the last cell. */
  grid-column: -2;
  grid-row: -2;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
