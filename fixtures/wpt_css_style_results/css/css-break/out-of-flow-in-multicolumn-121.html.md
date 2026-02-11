# css/css-break/out-of-flow-in-multicolumn-121.html

```json
{
  "format_version": 3,
  "file": "css/css-break/out-of-flow-in-multicolumn-121.html"
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
#nestedA {
  /* This nested multicol contains the position:relative abspos CB */
  width: 100%;
  height: 200px;
}
#abscb {
  position: relative;
  width: 100%;
  height: 400px;
}
.abspos {
  position: absolute;
  width: 100%;
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
