# css/css-text/letter-spacing/letter-spacing-ligatures-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/letter-spacing/letter-spacing-ligatures-001.html"
}
```

## style[0]

```css

@font-face {
    font-family: 'mplus';
    src: url('/fonts/mplus-1p-regular.woff');
}
div {
    font-size: 2em;
    font-family: mplus;
    width: 12ch;
    text-align: justify;
    text-align-last: justify;
    text-justify: inter-character;
    font-kerning: none;
}
span {
    display: inline-block;
    text-justify: auto;
}
.ref { color: blue; font-variant-ligatures: none; }
.noref { color: orange; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
