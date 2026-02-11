# css/css-fonts/font-variant-position-03.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variant-position-03.html"
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
	 font-variant-position: super;
  }
  .low {
	font-feature-settings: "subs" off, "sups" on;
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
