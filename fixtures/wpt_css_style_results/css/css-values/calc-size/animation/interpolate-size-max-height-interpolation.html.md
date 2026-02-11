# css/css-values/calc-size/animation/interpolate-size-max-height-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-size/animation/interpolate-size-max-height-interpolation.html"
}
```

## style[0]

```css

:root {
  interpolate-size: allow-keywords;
}
.parent {
  height: 200px;
  max-height: 300px;
}
.target {
  width: 100px;
  height: 50px;
  max-height: 100px;
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
