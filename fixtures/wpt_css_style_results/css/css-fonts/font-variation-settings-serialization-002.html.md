# css/css-fonts/font-variation-settings-serialization-002.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variation-settings-serialization-002.html"
}
```

## style[0]

```css

  @font-face {
    font-family: "Roboto";
    src: url('support/fonts/RobotoExtremo-VF.subset.ttf') format('truetype');
  }
  .test1 {
    font-family: "Roboto";
    font-variation-settings: 'wdth' 125, 'wght' 400, 'opsz' 144;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “font-variation-settings”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
