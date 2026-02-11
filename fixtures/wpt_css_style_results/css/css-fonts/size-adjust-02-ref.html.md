# css/css-fonts/size-adjust-02-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/size-adjust-02-ref.html"
}
```

## style[0]

```css

@font-face {
  font-family: reference-font;
  src: local(Ahem), url(/fonts/Ahem.ttf);
  unicode-range: U+41-5A; /* Uppercase ASCII only */
}

div {
  font-size: 40px;
  line-height: 100px;
  font-size-adjust: 0.5;
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
