# css/css-fonts/font-face-unicode-range-2.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-face-unicode-range-2.html"
}
```

## style[0]

```css

  @font-face {
    font-family: base;
    src: url(support/fonts/LigatureSymbolsWithSpaces.woff);
  }
  @font-face {
    font-family: swoosh;
    src: url(support/fonts/Rochester.otf);
    unicode-range: U+26;
  }
  .test {
    font-family: swoosh, base;
  }
  .ref {
    font-family: base;
  }
  .ref .amp {
	  font-family: swoosh, base;
  }
  div {
	  font-size: 5em;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
