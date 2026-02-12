# css/css-contain/contain-html-dir-004.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-html-dir-004.html"
}
```

## style[0]

```css

html::before {
    content: "";
    width: 100px;
    height: 100px;
    background: orange;
    display: block;
}
p {
    margin: 0;
    direction: ltr;
}
body {
    margin: 0 auto 0 0;
    width: 200px;
    height: 200px;
    direction: rtl;
}
html {
    contain: style;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
