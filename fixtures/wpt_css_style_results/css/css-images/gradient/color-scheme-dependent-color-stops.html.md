# css/css-images/gradient/color-scheme-dependent-color-stops.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/color-scheme-dependent-color-stops.html"
}
```

## style[0]

```css


.dark {
    color-scheme: dark;
}

.box {
    width: 100px;
    height: 100px;
}

#system-color {
    background-image: linear-gradient(CanvasText, CanvasText);
}

#system-color-in-color-mix {
    background-image: linear-gradient(color-mix(in lch, Canvas, pink), color-mix(in lch, Canvas, pink));
}

#light-dark {
    background-image: linear-gradient(light-dark(red, green), light-dark(red, green));
}

#light-dark-in-color-mix {
    background-image: linear-gradient(color-mix(in lch, light-dark(red, green), pink), color-mix(in lch, light-dark(red, green), pink));
}

```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
