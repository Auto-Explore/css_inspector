# css/css-fonts/font-variant-numeric-03.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variant-numeric-03.html"
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
	 font-variant-numeric: oldstyle-nums;
  }
  .low {
   font-feature-settings: "lnum" off, "onum" on, "pnum" off, "tnum" off, "frac" off, "afrc" off, "ordn" off, "zero" off;
  }


```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
