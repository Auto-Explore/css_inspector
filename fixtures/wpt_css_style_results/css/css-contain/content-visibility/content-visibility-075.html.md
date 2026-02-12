# css/css-contain/content-visibility/content-visibility-075.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-075.html"
}
```

## style[0]

```css

.auto {
  content-visibility: auto;
  contain-intrinsic-size: 1px 10000px;
}
.child {
  height: 40000px;
  position: relative;
}
#target {
  position: absolute;
  bottom: 0;
  font: 25px/1 Ahem;
}
.before_target {
  height: 40000px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
