# css/css-fonts/font-face-stretch-default-variable-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-face-stretch-default-variable-ref.html"
}
```

## style[0]

```css

    @font-face {
        font-family: "Roboto";
        src: url('support/fonts/RobotoExtremo-VF.subset.ttf') format('truetype');
        font-display: swap;
        font-style: normal;
        font-stretch: 75% 125%;
    }
    .stretch75 {
        font-family: "Roboto";
        font-size: 3em;
        font-variation-settings: 'wdth' 75;
    }
    .stretch100 {
        font-family: "Roboto";
        font-size: 3em;
        font-variation-settings: 'wdth' 100;
    }
    .stretch125 {
        font-family: "Roboto";
        font-size: 3em;
        font-variation-settings: 'wdth' 125;
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
