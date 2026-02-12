# css/css-values/calc-size/animation/interpolate-size-min-height-composition.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-size/animation/interpolate-size-min-height-composition.html"
}
```

## style[0]

```css

:root {
  interpolate-size: allow-keywords;
}
.parent {
  height: 200px;
}
.target {
  width: 100px;
  height: 150px;
}
.target::before {
  display: block;
  content: "";
  width: 100px;
  height: 50px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
