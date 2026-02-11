# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-auto-011-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-auto-011-ref.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  border: solid thick;
  margin: 10px;
  grid-template-rows: repeat(auto-fill, 100px 100px);
  grid-template-columns: auto auto;
  grid-row-gap: 100px;
  height: 300px;
  width: min-content;
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
