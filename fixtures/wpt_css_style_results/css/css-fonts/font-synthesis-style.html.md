# css/css-fonts/font-synthesis-style.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-synthesis-style.html"
}
```

## style[0]

```css

    @font-face {
        font-family: "lato";
        src: url(/fonts/Lato-Medium.ttf);
    }
    @supports not (font-synthesis-style: none) {
        .test {color: red;}
    }
    .test {
        font-family: "lato";
        font-size: 3em;
    }
    .nosynth {
        font-style: italic;
        font-synthesis-style: none;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
