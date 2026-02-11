# css/css-fonts/font-palette-33.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-palette-33.html"
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

@font-palette-values --MyPalette {
    font-family: "CoLr-tEsT-FoNt";
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
