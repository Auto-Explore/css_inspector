# css/css-fonts/font-kerning-02.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-kerning-02.html"
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
  }
  .high {
   font-kerning: normal;
  }
  .low {
   font-feature-settings: "kern" on;
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
