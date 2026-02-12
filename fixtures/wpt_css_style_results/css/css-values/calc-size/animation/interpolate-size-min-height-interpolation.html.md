# css/css-values/calc-size/animation/interpolate-size-min-height-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-size/animation/interpolate-size-min-height-interpolation.html"
}
```

## style[0]

```css

:root {
  interpolate-size: allow-keywords;
}
.parent {
  min-height: 30px;
}
.target {
  width: 10px;
  height: 0px;
  min-height: 10px;
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
