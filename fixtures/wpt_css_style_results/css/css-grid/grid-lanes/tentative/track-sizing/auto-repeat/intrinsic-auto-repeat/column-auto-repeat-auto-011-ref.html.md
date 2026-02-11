# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-011-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/column-auto-repeat-auto-011-ref.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  border: solid thick;
  margin: 10px;
  grid-template-columns: repeat(auto-fill, 100px 100px);
  grid-column-gap: 100px;
  width: 300px;
  background: pink;
}
.grid > div {
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
