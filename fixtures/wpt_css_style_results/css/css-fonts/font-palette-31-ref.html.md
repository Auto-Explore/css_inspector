# css/css-fonts/font-palette-31-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-palette-31-ref.html"
}
```

## style[0]

```css

@font-face {
    font-family: "COLR-test-font";
    src: url("resources/COLR-palettes-test-font.ttf") format("truetype");
}

@font-palette-values --MyPalette1 {
    font-family: "COLR-test-font";
    base-palette: 5;
}

@font-palette-values --MyPalette2 {
    font-family: "COLR-test-font";
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
