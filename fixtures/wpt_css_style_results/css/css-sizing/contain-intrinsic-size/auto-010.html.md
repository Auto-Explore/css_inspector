# css/css-sizing/contain-intrinsic-size/auto-010.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/auto-010.html"
}
```

## style[0]

```css

#wrapper {
  column-width: 100px;
  width: max-content;
  height: 100px;
}
#target {
  width: max-content;
  height: max-content;
  background: orange;
}
#target::before {
  content: "";
  display: block;
}
.content-50-150::before {
  width: 50px;
  height: 150px;
}
.content-50-175::before {
  width: 50px;
  height: 175px;
}
.content-skip {
  content-visibility: hidden;
}
.cis-auto {
  contain-intrinsic-size: auto 1px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
