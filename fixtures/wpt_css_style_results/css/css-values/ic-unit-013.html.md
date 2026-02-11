# css/css-values/ic-unit-013.html

```json
{
  "format_version": 3,
  "file": "css/css-values/ic-unit-013.html"
}
```

## style[0]

```css

/* The following font contains the CJK water (U+6C34) glyph as a zero-width character. */
@font-face {
  font-family: IcTestZeroWidth;
  src: url(resources/IcTestZeroWidth.woff2);
}

.test {
    font: 20px IcTestZeroWidth;
    width: calc(100px + 10ic);
    height: 20px;
    background: green;
    margin-bottom: 10px;
}

.ref {
    /* Each ic should be equal to 0px because the CJK water glyph in the font is zero-width. */
    width: 100px;
    height: 20px;
    background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
