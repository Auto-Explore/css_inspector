# css/css-text/letter-spacing/reference/letter-spacing-ligatures-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text/letter-spacing/reference/letter-spacing-ligatures-001-ref.html"
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
    font-variant-ligatures: none;
    font-kerning: none;
}
span {
    display: inline-block;
    text-justify: auto;
}
.ref { color: blue;}
.noref { color: orange; font-variant-ligatures: initial;}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
