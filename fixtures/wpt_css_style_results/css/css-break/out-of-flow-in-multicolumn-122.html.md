# css/css-break/out-of-flow-in-multicolumn-122.html

```json
{
  "format_version": 3,
  "file": "css/css-break/out-of-flow-in-multicolumn-122.html"
}
```

## style[0]

```css

.multicol {
  column-count: 2;
  column-fill: auto;
  column-gap: 0;
}
#outer {
  width: 100px;
  height: 100px;
  background: red;
}
#abscb {
  position: relative;
  width: 100%;
  height: 200px;
}
#nestedB {
  /* This nested multicol is between the abspos element and its CB */
  width: 100%;
  height: 100px;
}
.abspos {
  position: absolute;
  width: 50px;
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
