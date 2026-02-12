# css/css-values/calc-size/animation/interpolate-size-min-width-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-size/animation/interpolate-size-min-width-interpolation.html"
}
```

## style[0]

```css

:root {
  interpolate-size: allow-keywords;
}
.parent {
  min-width: 30px;
}
.target {
  width: 0px;
  height: 10px;
  min-width: 10px;
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
