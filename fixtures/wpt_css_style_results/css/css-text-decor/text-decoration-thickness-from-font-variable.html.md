# css/css-text-decor/text-decoration-thickness-from-font-variable.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-thickness-from-font-variable.html"
}
```

## style[0]

```css

@font-face {
font-family: underline-variable;
src: url(resources/UnderlineTest-VF.ttf);
}

.test {
text-underline-position: from-font;
font-size: 64px;
line-height: 1.8;
}

.thin_underline {
text-decoration: underline;
text-decoration-thickness: from-font;
font-family: underline-variable, sans-serif;
font-variation-settings: 'UNDS' 1;
}

.thick_underline {
text-decoration: underline;
text-decoration-thickness: from-font;
font-family: underline-variable, sans-serif;
font-variation-settings: 'UNDS' 1000;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
