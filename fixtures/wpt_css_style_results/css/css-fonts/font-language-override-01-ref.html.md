# css/css-fonts/font-language-override-01-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-language-override-01-ref.html"
}
```

## style[0]

```css

@font-face {
  font-family: Libertine;
  src: url(support/fonts/LinLibertine_Re-4.7.5.woff);
}
body {
  font-family: sans-serif;
}
div {
  margin: 1em;
  font: 32px Libertine;
}
.ref {
  font-feature-settings: "liga" 0;
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
