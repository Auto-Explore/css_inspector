# css/css-multicol/crashtests/multicol-dynamic-contain-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/crashtests/multicol-dynamic-contain-crash.html"
}
```

## style[0]

```css

#multicol {
  columns: 2;
  column-fill: auto;
  height: 50px;
  background: orange;
}
#container {
  contain: strict;
  width: 100px;
  height: 100px;
  background: blue;
}
#target {
  width: 100px;
  height: 100px;
  position: absolute;
  background: purple;
}
.transform {
  transform: translate(0, 50px);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
