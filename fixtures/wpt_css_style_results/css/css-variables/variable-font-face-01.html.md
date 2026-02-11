# css/css-variables/variable-font-face-01.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-font-face-01.html"
}
```

## style[0]

```css

@font-face {
  --a: MyTestFontName;
  font-family: var(--a);
  src: url(/fonts/Ahem.ttf);
}
@font-face {
  font-family: MyTestFontName2;
  src: url(/fonts/Ahem.ttf);
}
#a {
  font-family: MyTestFontName, serif;
}
#b {
  font-family: MyTestFontName2, serif;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
