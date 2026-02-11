# css/css-values/calc-size/animation/interpolate-size-width-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-size/animation/interpolate-size-width-interpolation.html"
}
```

## style[0]

```css

:root {
  interpolate-size: allow-keywords;
}
.parent {
  width: 150px;
  overflow: visible;
}
.target {
  width: 50px;
  height: 10px;
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
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “interpolate-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
