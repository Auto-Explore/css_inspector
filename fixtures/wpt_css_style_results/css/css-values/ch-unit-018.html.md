# css/css-values/ch-unit-018.html

```json
{
  "format_version": 3,
  "file": "css/css-values/ch-unit-018.html"
}
```

## style[0]

```css

@font-face {
  font-family: NoDigits;
  src: local(Ahem), url("/fonts/Ahem.ttf");
  unicode-range: U+0000-002F, U+003A-007F;
}
@font-face {
  font-family: ChTestZeroWidthZero;
  src: url("resources/ChTestZeroWidthZero.woff");
}
div {
  height: 10px;
  background-color: blue;
  margin-top: 10px;
}
.test {
  width: calc(100px + 5ch);
}
.ref {
  width: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
