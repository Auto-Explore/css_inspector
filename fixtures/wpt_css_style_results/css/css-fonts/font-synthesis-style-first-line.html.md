# css/css-fonts/font-synthesis-style-first-line.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-synthesis-style-first-line.html"
}
```

## style[0]

```css

    @font-face {
        font-family: "Lato-Medium";
        src: url(/fonts/Lato-Medium.ttf);
    }
    @supports not (font-synthesis-style: none) {
        .test {color: red;}
    }
    .test {
        font-family: "Lato-Medium";
        font-size: 3em;
    }
    p.nosynth::first-line {
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
