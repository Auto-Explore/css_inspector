# css/css-fonts/font-palette-31.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-palette-31.html"
}
```

## style[0]

```css

@font-face {
    font-family: "COLR-test-font1";
    src: url("resources/COLR-palettes-test-font.ttf") format("truetype");
    unicode-range: U+41;
}

@font-face {
    font-family: "COLR-test-font2";
    src: url("resources/COLR-palettes-test-font.ttf") format("truetype");
}

@font-palette-values --MyPalette {
    font-family: "COLR-test-font1";
    base-palette: 5;
}

@font-palette-values --MyPalette {
    font-family: "COLR-test-font2";
    base-palette: 2;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
