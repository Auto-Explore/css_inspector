# css/css-fonts/size-adjust-01-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/size-adjust-01-ref.html"
}
```

## style[0]

```css

@font-face {
  font-family: reference-font;
  src: local(Ahem), url(/fonts/Ahem.ttf);
  /* Uppercase ASCII only + U+20 to make this "first available font" */
  unicode-range: U+20,U+41-5A;
}

div {
  font-size: 40px;
  line-height: 100px;
}

.reference {
  font-family: reference-font, sans-serif;
}

.large {
  font-size: 60px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
