# css/css-fonts/font-variant-position-04.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-variant-position-04.html"
}
```

## style[0]

```css

    /* Lato has superscript Latin letters, but not subscript ones;
     * digits are available in both super- and subscript form.
     */
    @font-face {
        font-family: "lato";
        src: url(/fonts/Lato-Medium.ttf);
    }
    .test {
        font-family: "lato";
        font-size: 2em;
    }
    .sub {
        font-variant-position: sub;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
