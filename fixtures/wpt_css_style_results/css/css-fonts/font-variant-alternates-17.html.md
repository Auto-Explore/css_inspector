# css/css-fonts/font-variant-alternates-17.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variant-alternates-17.html"
}
```

## style[0]

```css

  @font-face {
    font-family: fwf;
    src: url(support/fonts/FontWithFancyFeatures.otf);
  }
  @font-feature-values fwf {
    @ornaments {
      foo: 1;
      bar: 2;
      baz: 3;
    }
  }
  .test {
	  font-family: fwf;
	  font-size: 2em;
	  line-height: 1.1;
  }
  .high {
	 font-variant-alternates: ornaments(baz);
  }
  .low {
   font-feature-settings: "hist" off, "salt" 00, "ss01" off, "ss02" off, "ss03" off,
    "cv01" off, "cv02" off, "cv03" off,  "swsh" 00, "cswh" 00, "ornm" 03, "nalt" 00;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
