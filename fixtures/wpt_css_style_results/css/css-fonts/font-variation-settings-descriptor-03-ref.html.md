# css/css-fonts/font-variation-settings-descriptor-03-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variation-settings-descriptor-03-ref.html"
}
```

## style[0]

```css

  @font-face {
    font-family: "Roboto";
    src: url('support/fonts/RobotoExtremo-VF.subset.ttf') format('truetype');
  }
  .weight100 {
     font-family: "Roboto";
     font-variation-settings: 'wdth' 125, 'opsz' 144, 'wght' 100;
  }
  .weight400 {
     font-family: "Roboto";
     font-variation-settings: 'wdth' 125, 'opsz' 144, 'wght' 400;
  }
  .weight900 {
     font-family: "Roboto";
     font-variation-settings: 'wdth' 125, 'opsz' 144, 'wght' 900;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “font-variation-settings”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-variation-settings”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-variation-settings”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
