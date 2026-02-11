# css/css-values/calc-size/animation/interpolate-size-width-composition.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-size/animation/interpolate-size-width-composition.html"
}
```

## style[0]

```css

:root {
  interpolate-size: allow-keywords;
}
.parent {
  width: 200px;
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
