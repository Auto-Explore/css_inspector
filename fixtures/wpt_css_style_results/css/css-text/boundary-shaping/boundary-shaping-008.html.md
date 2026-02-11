# css/css-text/boundary-shaping/boundary-shaping-008.html

```json
{
  "format_version": 3,
  "file": "css/css-text/boundary-shaping/boundary-shaping-008.html"
}
```

## style[0]

```css

@font-face {
  font-family: test;
  src: url(resources/LinLibertine_Re-4.7.5.woff);
}
body {
  font: 36px test; /* use a font that includes ligatures for "fi" etc */
}
.a {
  unicode-bidi: isolate; /* bidi isolation boundaries should break shaping */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
