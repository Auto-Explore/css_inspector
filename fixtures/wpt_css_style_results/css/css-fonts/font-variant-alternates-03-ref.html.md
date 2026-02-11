# css/css-fonts/font-variant-alternates-03-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variant-alternates-03-ref.html"
}
```

## style[0]

```css

  @font-face {
    font-family: fwf;
    src: url(support/fonts/FontWithFancyFeatures.otf);
  }
  @font-feature-values fwf {
    @stylistic {
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
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
