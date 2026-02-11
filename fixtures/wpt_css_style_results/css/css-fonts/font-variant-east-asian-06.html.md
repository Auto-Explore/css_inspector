# css/css-fonts/font-variant-east-asian-06.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variant-east-asian-06.html"
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
	 font-variant-east-asian: simplified;
  }
  .low {
   font-feature-settings: "jp78" off, "jp83" off, "jp90" off, "jp04" off, "smpl" on,  "trad" off, "fwid" off, "pwid" off, "ruby" off ;
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
