# css/css-color/display-p3-linear-006.html

```json
{
  "format_version": 3,
  "file": "css/css-color/display-p3-linear-006.html"
}
```

## style[0]

```css

    .test, .test2 { background-color: red; width: 12em; height: 4em; }
    .ref {background-color: rgb(44.8436% 53.537% 28.8112%); width: 12em; height: 4em; }
        /* lch(54% 35 118) converted to legacy sRGB */
    .test { background-color: color(display-p3-linear 0.183382 0.245634 0.082317); }
        /* lch(54% 35 118) converted to display-p3-linear */
    .test2 {background-color: color(srgb 0.448436 0.53537 0.288113); }
        /* lch(54% 35 118) converted to color(sRGB) */
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
