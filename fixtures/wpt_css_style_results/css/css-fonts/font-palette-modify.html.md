# css/css-fonts/font-palette-modify.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-palette-modify.html"
}
```

## style[0]

```css

@font-face {
    font-family: "COLR-test-font";
    src: url("resources/COLR-palettes-test-font.ttf") format("truetype");
}

@font-palette-values --MyPalette {
    font-family: "COLR-test-font";
    base-palette: 1;
}

@font-palette-values --MyPalette2 {
    font-family: "COLR-test-font";
    base-palette: 0;
    /* Glyph 'A' uses palette indices 3 and 7. */
    override-colors: 3 #00FF00;
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
