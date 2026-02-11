# css/css-fonts/size-adjust-tentative-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/size-adjust-tentative-ref.html"
}
```

## style[0]

```css

@font-face {
  font-family: custom-font;
  src: local(Ahem), url(/fonts/Ahem.ttf);
  unicode-range: U+20, U+30-39; /* Digits and space */
}

.target {
  font-size: 10px;
  font-family: custom-font, sans-serif;
}

.unaffected {
  font-size: 20px;
}

.square {
  display: inline-block;
  vertical-align: bottom;
  background-color: black;
  width: var(--l);
  height: var(--l);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
