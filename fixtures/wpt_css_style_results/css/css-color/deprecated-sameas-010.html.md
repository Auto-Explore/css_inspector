# css/css-color/deprecated-sameas-010.html

```json
{
  "format_version": 3,
  "file": "css/css-color/deprecated-sameas-010.html"
}
```

## style[0]

```css

    .test { background-color: red; width: 12em; height: 6em; }
    .ref { background-color: GrayText; width: 12em; height: 6em; }
    @supports (color: InactiveCaptionText) {
        .test { background-color: InactiveCaptionText; }
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
