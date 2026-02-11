# css/css-fonts/font-face-weight-default-variable-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-face-weight-default-variable-ref.html"
}
```

## style[0]

```css

    @font-face {
        font-family: "Roboto";
        src: url('support/fonts/RobotoExtremo-VF.subset.ttf') format('truetype');
        font-display: swap;
        font-style: normal;
        font-weight: 100 900;
    }
    .weight100 {
        font-family: "Roboto";
        font-size: 3em;
        font-variation-settings: 'wght' 100;
    }
    .weight400 {
        font-family: "Roboto";
        font-size: 3em;
        font-variation-settings: 'wght' 400;
    }
    .weight900 {
        font-family: "Roboto";
        font-size: 3em;
        font-variation-settings: 'wght' 900;
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
