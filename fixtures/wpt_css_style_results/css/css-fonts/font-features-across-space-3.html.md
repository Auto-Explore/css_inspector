# css/css-fonts/font-features-across-space-3.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-features-across-space-3.html"
}
```

## style[0]

```css

@font-face {
  font-family: ligsym;
  src: url(support/fonts/LigatureSymbolsWithSpaces.woff);
  font-feature-settings: "liga" 0;
}

td {
  padding: 10px 5px;
}

.test {
  font-family: ligsym;
  font-size: 150%;
  font-variant-ligatures: none;
  font-feature-settings: "liga" on;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “font-feature-settings”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
