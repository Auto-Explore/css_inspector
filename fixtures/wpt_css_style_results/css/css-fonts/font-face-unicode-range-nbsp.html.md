# css/css-fonts/font-face-unicode-range-nbsp.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-face-unicode-range-nbsp.html"
}
```

## style[0]

```css

@font-face {
  font-family: test;
  src: url(/fonts/Ahem.ttf);
  unicode-range: U+00A0;
  /* override metrics to ensure they match across the fonts: */
  ascent-override: 80%;
  descent-override: 20%;
}
@font-face {
  font-family: fallback;
  src: url(/fonts/GentiumPlus-R.woff);
  ascent-override: 80%;
  descent-override: 20%;
}
#test, #ref {
  font: 24px/2 test, fallback, serif;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
