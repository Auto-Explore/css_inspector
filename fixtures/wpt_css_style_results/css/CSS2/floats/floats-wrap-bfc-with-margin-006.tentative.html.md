# css/CSS2/floats/floats-wrap-bfc-with-margin-006.tentative.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats/floats-wrap-bfc-with-margin-006.tentative.html"
}
```

## style[0]

```css

.wrapper {
  width: 50px;
  margin-left: 50px;
  background: red;
}
.float {
  float: right;
  clear: right;
  width: 25px;
  height: 50px;
  background: green;
}
.bfc {
  overflow: hidden;
  height: 50px;
  margin-left: -50px;
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
