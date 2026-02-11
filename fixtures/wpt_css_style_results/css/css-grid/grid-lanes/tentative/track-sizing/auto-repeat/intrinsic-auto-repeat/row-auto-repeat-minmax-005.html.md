# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-minmax-005.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-minmax-005.html"
}
```

## style[0]

```css

.grid-lanes {
  display: grid-lanes;
  border: solid thick;
  margin: 10px;
  grid-template-rows: repeat(auto-fill, minmax(auto, auto) minmax(auto, auto));
  grid-row-gap: 100px;
  height: 300px;
  width: min-content;
  background: pink;
}
.grid-lanes > div {
  background: lime;
  width: 50px;
  height: 50px;
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
