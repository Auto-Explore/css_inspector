# css/css-fonts/variations/font-slant-1-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/variations/font-slant-1-ref.html"
}
```

## style[0]

```css

@font-face {
  font-family: test;
  font-style: oblique 10deg;
  src: url(resources/Inter.var.subset.ttf);
}
.test {
  font: 32px/1.5 test;
  font-variation-settings: 'slnt' -10;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “font-variation-settings”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
