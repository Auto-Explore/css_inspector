# css/css-fonts/font-palette-non-ident-font-family.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-palette-non-ident-font-family.html"
}
```

## style[0]

```css

@font-face {
    font-family: "foo bar";
    src: url("resources/COLR-palettes-test-font.ttf") format("truetype");
}

@font-palette-values --MyPalette {
    font-family: "foo bar";
    base-palette: 1;
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
