# css/css-fonts/parsing/font-size-adjust-computed.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/parsing/font-size-adjust-computed.html"
}
```

## style[0]

```css

/* Use a font with known metrics so we can verify that from-font
   computes to the expected value from the font. */
@font-face {
    font-family: ahem-ex-500;
    src: url(ahem-ex-500.otf);
}
#target {
    font-family: ahem-ex-500 !important;
    font-size: 1000px;
}
#container {
    container-type: inline-size;
    width: 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
