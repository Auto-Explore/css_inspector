# css/css-fonts/font-synthesis-small-caps-first-letter.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-synthesis-small-caps-first-letter.html"
}
```

## style[0]

```css

    @font-face {
        font-family: "Lato-Medium";
        src: url(/fonts/Lato-Medium.ttf);
    }
    @supports not (font-synthesis-small-caps: none) {
        .test {color: red;}
    }
    .test {
        font-family: "Lato-Medium";
        font-size: 3em;
    }
    p.nosynth::first-letter {
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
