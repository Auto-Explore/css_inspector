# css/css-fonts/font-variant-alternates-01.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variant-alternates-01.html"
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
	 font-variant-alternates: normal;
  }
  .low {
   font-feature-settings: "hist" off, "salt" 00, "ss01" off, "ss02" off, "ss03" off,
    "cv01" off, "cv02" off, "cv03" off,  "swsh" 00, "cswh" 00, "ornm" 00, "nalt" 00;
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
