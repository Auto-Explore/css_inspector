# css/css-break/out-of-flow-in-multicolumn-040.html

```json
{
  "format_version": 3,
  "file": "css/css-break/out-of-flow-in-multicolumn-040.html"
}
```

## style[0]

```css

.multicol {
  column-count: 2;
  column-fill: auto;
  column-gap: 0px;
}
#outer {
  height: 100px;
  width: 100px;
  margin-left: -100px;
}
#inner {
  width: 50px;
  height: 700px;
  margin-left: -425px;
}
.rel {
  position: relative;
}
.abs {
  position: absolute;
}
.fixed {
  position: fixed;
  width: 50px;
  height: 200px;
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
