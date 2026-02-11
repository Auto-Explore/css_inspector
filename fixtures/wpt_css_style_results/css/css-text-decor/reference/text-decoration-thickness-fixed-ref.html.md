# css/css-text-decor/reference/text-decoration-thickness-fixed-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/reference/text-decoration-thickness-fixed-ref.html"
}
```

## style[0]

```css

@font-face {
font-family: underline-thin;
src: url(../resources/UnderlineTest-Thin.ttf);
}

@font-face {
font-family: underline-thick;
src: url(../resources/UnderlineTest-Thick.ttf);
}

.test {
text-underline-position: from-font;
font-size: 64px;
line-height: 1.8;
}

.thin_underline {
text-decoration: underline;
font-family: underline-thin;
text-decoration-thickness: from-font;
}

.thick_underline {
text-decoration: underline;
font-family: underline-thick;
text-decoration-thickness: from-font;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
