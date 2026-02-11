# css/css-grid/layout-algorithm/grid-intrinsic-track-sizes-min-size-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/grid-intrinsic-track-sizes-min-size-001.html"
}
```

## style[0]

```css

.grid{
  display: inline-grid;
  grid-template-columns: minmax(auto,0) minmax(auto,0);
  background: red;
  height: 100px;
}
.t{
  grid-column: 1 / span 2;
  border-inline: 50px solid green;
  min-width: 0;
}
.inner{
  display: inline-block;
  width: 100px;
  height: 10px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-inline”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
