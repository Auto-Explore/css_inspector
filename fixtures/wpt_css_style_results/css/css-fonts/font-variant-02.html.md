# css/css-fonts/font-variant-02.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variant-02.html"
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
	  font-size: 2.4em;
	  line-height: 1.1;
    color: purple;
  }
  .inner {
	  font-variant: none;
  }
  .outer {
    font-variant-ligatures: common-ligatures  discretionary-ligatures historical-ligatures contextual;
    font-variant-numeric: oldstyle-nums;
    font-variant-caps: small-caps;
    font-variant-east-asian: jis90;
  }
  .child {
    color: green;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “font-variant-ligatures”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
