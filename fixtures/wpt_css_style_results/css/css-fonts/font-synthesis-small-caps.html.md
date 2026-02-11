# css/css-fonts/font-synthesis-small-caps.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-synthesis-small-caps.html"
}
```

## style[0]

```css

    @font-face {
        font-family: "lato";
        src: url(/fonts/Lato-Medium.ttf);
    }
    @supports not (font-synthesis-small-caps: none) {
        .test {color: red;}
    }
    .test {
        font-family: "lato";
        font-size: 3em;
    }
    .nosynth {
        font-variant: small-caps;
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
