# css/css-fonts/font-face-style-default-variable-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-face-style-default-variable-ref.html"
}
```

## style[0]

```css

    @font-face {
        font-family: "Inter";
        src: url('support/fonts/Inter-VF.subset.ttf') format('truetype');
        font-display: swap;
        font-style: oblique 10deg 0deg;
    }
    .style10 {
        font-family: "Inter";
        font-size: 3em;
        font-variation-settings: 'slnt' -10;
    }
    .style0 {
        font-family: "Inter";
        font-size: 3em;
        font-variation-settings: 'slnt' 0;
    }
```

```json
{
  "errors": 2,
  "messages": [
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
