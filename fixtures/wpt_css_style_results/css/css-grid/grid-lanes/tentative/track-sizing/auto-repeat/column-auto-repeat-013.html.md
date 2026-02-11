# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-013.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-013.html"
}
```

## style[0]

```css

.grid-lanes {
  display: grid-lanes;
  border: solid thick;
  margin: 10px;
  grid-template-columns: repeat(auto-fill, 50px 50px);
  grid-column-gap: 100px;
  width: 300px;
  background: pink;
}
.grid-lanes > div {
  background: lime;
  width: 100%;
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
