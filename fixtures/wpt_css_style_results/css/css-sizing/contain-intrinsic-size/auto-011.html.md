# css/css-sizing/contain-intrinsic-size/auto-011.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/auto-011.html"
}
```

## style[0]

```css

#target {
  width: max-content;
  height: max-content;
  border: 1px solid;
}
#target::before {
  content: "";
  display: block;
}
.content-100-50::before {
  width: 100px;
  height: 50px;
}
.content-skip {
  content-visibility: hidden;
}
.contain-size {
  contain: size;
}
.contain-inline-size {
  contain: inline-size;
}
.ciw-auto-2 {
  contain-intrinsic-width: auto 2px;
}
.ciw-auto-20 {
  contain-intrinsic-width: auto 20px;
}
.cih-auto-1 {
  contain-intrinsic-height: auto 1px;
}
.cih-auto-10 {
  contain-intrinsic-height: auto 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
