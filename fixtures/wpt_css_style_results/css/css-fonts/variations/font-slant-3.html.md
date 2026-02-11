# css/css-fonts/variations/font-slant-3.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/variations/font-slant-3.html"
}
```

## style[0]

```css

@font-face {
  font-family: test;
  /* The font resource includes a 'slnt' axis, but our font-style descriptor
     should prevent it being used to render oblique/italic styles. */
  src: url(resources/Inter.var.subset.ttf);
  font-style: normal;
  font-weight: normal;
  font-stretch: normal;
}
.test {
  font-synthesis: style;
  font: 32px/1.5 test;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
