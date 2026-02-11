# css/css-text-decor/reference/text-decoration-inset-011-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/reference/text-decoration-inset-011-ref.html"
}
```

## style[0]

```css

body {
  background: white;
  color: black;
  font-family: times new roman, serif;
}
div {
  position: relative;
  margin-left: 10px;
}
h1 {
  position: absolute;
  line-height: 2;
  width: 10em;
}
u.a {
  text-decoration-color: blue;
  text-underline-offset: 20px;
}
u.b {
  text-decoration-color: green;
  text-underline-offset: 20px;
}
span {
  display: inline-block;
  position: relative;
}
span u.a, span u.b {
  display: inline-block;
  width: calc(100% - 10px);
  overflow-x: clip;
  position: absolute;
  left: 0;
}
span u.a2 {
  left: auto;
  right: 0;
}
span u.b2 {
  width: 100%;
}
span u.b3 {
  width: 100%;
  left: -10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
