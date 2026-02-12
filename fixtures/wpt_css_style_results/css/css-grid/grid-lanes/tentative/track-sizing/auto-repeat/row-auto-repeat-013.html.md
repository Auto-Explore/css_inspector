# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-013.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-013.html"
}
```

## style[0]

```css

.grid-lanes {
  display: grid-lanes;
  border: solid thick;
  grid-lanes-direction: row;
  grid-template-rows: 10px 20px repeat(auto-fill, 30px 40px) 50px 60px;
  height: 300px;
  background: pink;
}
.grid-lanes > :nth-child(2n) {
  background: sienna;
}
.grid-lanes > :nth-child(2n+1) {
  background: orange;
}
.grid-lanes > div {
  width: 25px;
  height: 100%;
}
.holder {
  height: 300px;
  width: 50px;
  border-bottom: 2px solid #cfbfcf;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
