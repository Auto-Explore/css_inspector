# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-013-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/row-auto-repeat-013-ref.html"
}
```

## style[0]

```css

.grid {
  display: grid;
  border: solid thick;
  grid-template-rows: 10px 20px repeat(auto-fill, 30px 40px) 50px 60px;
  height: 300px;
  background: pink;
}
.grid > :nth-child(2n) {
  background: sienna;
}
.grid > :nth-child(2n+1) {
  background: orange;
}
.grid > div {
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
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
