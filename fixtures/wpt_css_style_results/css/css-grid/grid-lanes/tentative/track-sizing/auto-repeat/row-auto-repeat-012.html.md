# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-012.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-012.html"
}
```

## style[0]

```css

.grid-lanes {
  display: grid-lanes;
  border: solid thick;
  margin: 10px;
  grid-lanes-direction: row;
  grid-template-rows: repeat(auto-fill, 50px 50px);
  grid-row-gap: 100px;
  height: 300px;
  width: min-content;
  background: pink;
}
.grid-lanes > div {
  background: lime;
  width: 50px;
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
