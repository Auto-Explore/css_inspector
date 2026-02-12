# css/css-text-decor/reference/text-decoration-inset-018-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/reference/text-decoration-inset-018-ref.html"
}
```

## style[0]

```css

body {
  background: white;
  color: black;
}
div.outer {
  display: inline-block;
  vertical-align: top;
  font: 10px/2 Ahem;
  position: relative;
  width: 12ch;
  height: 12ch;
  border: 1px solid gray;
  margin: 1em;
}
div.inner {
  position: absolute;
}
div.inner > p {
  margin: 0;
  position: absolute;
  inline-size: 12ch;
}
p:dir(rtl) {
  unicode-bidi: bidi-override;
}
p.ul {
  color: transparent;
}
u {
  text-decoration: 2px black underline;
  text-underline-offset: 3px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
