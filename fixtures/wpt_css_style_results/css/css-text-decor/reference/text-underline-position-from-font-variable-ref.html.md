# css/css-text-decor/reference/text-underline-position-from-font-variable-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/reference/text-underline-position-from-font-variable-ref.html"
}
```

## style[0]

```css

@font-face {
font-family: underline-close;
src: url(../resources/UnderlineTest-Close.ttf);
}

@font-face {
font-family: underline-far;
src: url(../resources/UnderlineTest-Far.ttf);
}

.test {
text-underline-position: from-font;
font-size: 64px;
line-height: 1.8;
}

.close_underline {
text-decoration: underline;
font-family: underline-close;
}

.far_underline {
text-decoration: underline;
font-family: underline-far;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
