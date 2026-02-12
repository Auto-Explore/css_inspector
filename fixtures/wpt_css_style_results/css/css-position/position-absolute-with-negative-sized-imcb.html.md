# css/css-position/position-absolute-with-negative-sized-imcb.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-with-negative-sized-imcb.html"
}
```

## style[0]

```css

.rtl {
  direction: rtl;
}
.container {
  display: inline-block;
  margin: 8px;
  border: 1px solid;
  position: relative;
  width: 20px;
  height: 20px;
}
.abspos {
  position: absolute;
  background: orange;
}
.abspos.case1 {
  margin: auto;
  width: 18px;
  height: 18px;
}
.abspos.case2 {
  inset: 18px;
  margin: auto;
}
.abspos.case3 {
  inset: 18px;
  margin: -10px;
}
.abspos.case4 {
  width: 10px;
  height: 10px;
  place-self: unsafe center;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
