# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-014-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-014-ref.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  border: solid thick;
  grid-template-columns: 10px 20px repeat(auto-fill, 30px 40px) 50px 60px;
  width: 300px;
  background: pink;
}
.grid > :nth-child(2n) {
  background: sienna;
}
.grid > :nth-child(2n+1) {
  background: orange;
}
.grid > div {
  width: 100%;
  height: 25px;
}
.holder {
  height: 30px;
  width: 300px;
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
