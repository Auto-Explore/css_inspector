# css/css-fonts/font-variant-caps-07.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variant-caps-07.html"
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
	 font-variant-caps: titling-caps;
  }
  .low {
   font-feature-settings: "smcp" off, "c2sc" off, "pcap" off, "c2pc" off, "unic" off, "titl" on;
  }
.ref {

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
