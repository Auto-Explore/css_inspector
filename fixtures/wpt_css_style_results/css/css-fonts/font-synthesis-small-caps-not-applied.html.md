# css/css-fonts/font-synthesis-small-caps-not-applied.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-synthesis-small-caps-not-applied.html"
}
```

## style[0]

```css

    @font-face {
        font-family: "exo";
        src: url(support/fonts/Exo-DemiBold.otf);
    }
    @supports not (font-synthesis-small-caps: none) {
        .test {color: red;}
    }
    .test {
        font-family: "exo";
        font-size: 3em;
        font-variant: small-caps;
    }
    .nosynth {
        font-synthesis-small-caps: none;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
