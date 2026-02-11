# css/css-fonts/italic-oblique-fallback.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/italic-oblique-fallback.html"
}
```

## style[0]

```css

/* font-family test1 has 'normal' and 'oblique' (reduced-size) faces */
@font-face {
  font-family: test1;
  font-style: normal;
  src: url("resources/markA.ttf");
}
@font-face {
  font-family: test1;
  font-style: oblique;
  src: url("resources/markA.ttf");
  size-adjust: 50%;
}

/* font-family test2 has 'normal' and 'italic' (reduced-size) faces */
@font-face {
  font-family: test2;
  font-style: normal;
  src: url("resources/markA.ttf");
}
@font-face {
  font-family: test2;
  font-style: italic;
  src: url("resources/markA.ttf");
  size-adjust: 50%;
}

/* font-family test3 has only an 'italic' face */
@font-face {
  font-family: test3;
  font-style: italic;
  src: url("resources/markA.ttf");
}

div {
  font-size: 50px;
  font-synthesis: none;
}

.test1 {
  font-family: test1, serif;
}

.test2 {
  font-family: test2, serif;
}

.test3 {
  font-family: test3, serif;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “size-adjust”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “size-adjust”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
