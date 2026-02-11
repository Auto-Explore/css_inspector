# css/css-fonts/font-features-across-space-1-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-features-across-space-1-ref.html"
}
```

## style[0]

```css

@font-face {
  font-family: ligsym;
  src: url(support/fonts/LigatureSymbolsWithSpaces.woff);
}

td {
  padding: 10px 5px;
}

.test {
  font-family: ligsym;
  font-size: 150%;
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
