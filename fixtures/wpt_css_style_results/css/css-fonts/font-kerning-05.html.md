# css/css-fonts/font-kerning-05.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-kerning-05.html"
}
```

## style[0]

```css

  @font-face {
    font-family: fwf;
    src: url(support/fonts/FontWithFancyFeatures.otf);
  }
  .test {
    font-family: fwf;
    font-size: 2em;
    line-height: 1.1;
    writing-mode: vertical-rl;
    text-orientation: upright;
  }
  .high {
   font-kerning: none;
  }
  .low {
   font-feature-settings: "kern" off, "vkrn" off;
  }

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “font-feature-settings”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
