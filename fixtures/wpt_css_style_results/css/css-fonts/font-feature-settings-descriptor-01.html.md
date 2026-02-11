# css/css-fonts/font-feature-settings-descriptor-01.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-feature-settings-descriptor-01.html"
}
```

## style[0]

```css

  @font-face {
    font-family: fwf;
    src: url(support/fonts/FontWithFancyFeatures.otf);
  }
  @font-face {
    font-family: fwf2;
    src: url(support/fonts/FontWithFancyFeatures.otf);
    font-feature-settings: "liga" on, "clig" on, "calt" on, "hlig" on, "dlig" on, "onum" on, "smcp" on, "jp90" on;
  }
  .test {
	  font-family: fwf;
	  font-size: 2.4em;
	  line-height: 1.1;
  }
  .test>p.descriptor {
      font-family: fwf2;
  }
  .property {
    font-feature-settings: "liga" on, "clig" on, "calt" on, "hlig" on, "dlig" on, "onum" on, "smcp" on, "jp90" on;
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
