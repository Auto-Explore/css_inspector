# css/css-fonts/animations/font-palette-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/animations/font-palette-interpolation.html"
}
```

## style[0]

```css

@font-palette-values --custom-palette {
  font-family: "Color font";
  base-palette: 0;
  override-colors: 1 #000000;
}

.container {
  font-palette: light;
}
.container2 {
  font-palette: dark;
}
.target {
  display: inline-block;
  font: 100px sans-serif;
  font-palette: normal;
}
.expected {
  color: green;
  margin-right: 30px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “font-palette”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “font-palette”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “font-palette”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
