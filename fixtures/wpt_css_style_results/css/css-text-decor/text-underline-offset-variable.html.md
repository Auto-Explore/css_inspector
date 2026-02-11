# css/css-text-decor/text-underline-offset-variable.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-underline-offset-variable.html"
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

.close_underline {
text-decoration: underline;
font-family: underline-variable, sans-serif;
text-underline-offset: -2.5px;
}

.far_underline {
text-decoration: underline;
font-family: underline-variable, sans-serif;
text-underline-offset: 4.6px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
