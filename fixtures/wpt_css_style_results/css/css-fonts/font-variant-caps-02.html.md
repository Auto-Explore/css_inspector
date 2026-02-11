# css/css-fonts/font-variant-caps-02.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variant-caps-02.html"
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
	 font-variant-caps: small-caps;
  }
  .low {
   font-feature-settings: "smcp" on, "c2sc" off, "pcap" off, "c2pc" off, "unic" off, "titl" off;
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
