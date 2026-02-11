# css/css-position/absolute-pos-box-inside-fixed-pos-box-with-changing-height.html

```json
{
  "format_version": 3,
  "file": "css/css-position/absolute-pos-box-inside-fixed-pos-box-with-changing-height.html"
}
```

## style[0]

```css

body {
  margin: 0px;
}
#fixed {
  left: 0px;
  width: 100px;
  height: 200px;
  position: fixed;
}
.box {
  position: absolute;
  bottom: 0;
  width: 50px;
  height: 50px;
  background: green;
}
.ref {
  position: absolute;
  width: 50px;
  height: 50px;
  left: 0px;
  top: 250px;
  background: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
