# css/css-fonts/font-palette-36.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-palette-36.html"
}
```

## style[0]

```css

@font-face {
    font-family: "COLR-test-font";
    src: url("resources/COLR-palettes-test-font.ttf") format("truetype");
}
@font-face {
    font-family: "foo bar";
    src: url("resources/COLR-palettes-test-font.ttf") format("truetype");
}
@font-face {
    font-family: foo;
    src: url("resources/COLR-palettes-test-font.ttf") format("truetype");
}
@font-palette-values --MyPalette {
    font-family: "COLR-test-font", "foo bar", foo;
    base-palette: light;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
