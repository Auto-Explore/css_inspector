# css/css-color/color-mix-currentcolor-nested-for-color-property.html

```json
{
  "format_version": 3,
  "file": "css/css-color/color-mix-currentcolor-nested-for-color-property.html"
}
```

## style[0]

```css

body {
    color: color(srgb 0 0 1);
}
div {
    color: color-mix(in srgb, color-mix(in srgb, currentColor 50%, color(srgb 0 1 0)), white);
}
div > div {
    color: inherit;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
