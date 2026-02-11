# css/css-fonts/font-feature-settings-descriptor-02-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-feature-settings-descriptor-02-ref.html"
}
```

## style[0]

```css

  @font-face {
    font-family: "liga1";
    src: url('support/fonts/LigatureSymbolsWithSpaces.woff');
  }
  .ligaOFF {
    font-family: liga1;
    font-feature-settings: "liga" off;
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
