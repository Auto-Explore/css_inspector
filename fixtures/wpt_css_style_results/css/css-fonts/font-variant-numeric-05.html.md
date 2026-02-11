# css/css-fonts/font-variant-numeric-05.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variant-numeric-05.html"
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
	  font-size: 3em;
	  line-height: 1.1;
  }
  .high {
	 font-variant-numeric: tabular-nums;
  }
  .low {
   font-feature-settings: "lnum" off, "onum" off, "pnum" off, "tnum" on, "frac" off, "afrc" off, "ordn" off, "zero" off;
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
