# css/css-fonts/font-variant-ligatures-06.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variant-ligatures-06.html"
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
	font-variant-ligatures: no-discretionary-ligatures;
  }
  .low {
	font-feature-settings: "dlig" off;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
