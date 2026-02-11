# css/css-color/srgb-linear-003.html

```json
{
  "format_version": 3,
  "file": "css/css-color/srgb-linear-003.html"
}
```

## style[0]

```css

    body { background-color: grey; }
    .test { background-color: red; width: 12em; height: 6em; margin-top: 0; }
    .ref { background-color: color(srgb 1 1 1); width: 12em; height: 6em; margin-bottom: 0; } /* color(srgb-linear 1 1 1) converted to sRGB */
    .test { background-color: color(srgb-linear 1 1 1); }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
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
