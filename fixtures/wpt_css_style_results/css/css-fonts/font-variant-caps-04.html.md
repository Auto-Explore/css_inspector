# css/css-fonts/font-variant-caps-04.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variant-caps-04.html"
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
	 font-variant-caps: petite-caps;
  }
  .low {
   font-feature-settings: "smcp" off, "c2sc" off, "pcap" on, "c2pc" off, "unic" off, "titl" off;
  }
.ref {

}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
