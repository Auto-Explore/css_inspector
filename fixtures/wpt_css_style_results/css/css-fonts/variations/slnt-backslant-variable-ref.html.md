# css/css-fonts/variations/slnt-backslant-variable-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/variations/slnt-backslant-variable-ref.html"
}
```

## style[0]

```css

  @font-face {
    font-family: "slnt test font";
    src: url('resources/FontStyleTest-slnt_backslant-VF.woff2');
    font-style: oblique -15deg 15deg;
  }

  .test {
    font-family: "slnt test font";
    font-size: 3em;
  }

  .slnt14 {
    font-variation-settings: 'slnt' -14;
  }

  .backslant14 {
    font-variation-settings: 'slnt' 14;
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
