# css/css-fonts/font-synthesis-weight-first-line.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-synthesis-weight-first-line.html"
}
```

## style[0]

```css

    @font-face {
        font-family: "Lato-Medium";
        src: url(/fonts/Lato-Medium.ttf);
    }
    @supports not (font-synthesis-weight: none) {
        .test {color: red;}
    }
    .test {
        font-family: "Lato-Medium";
        font-size: 3em;
    }
    p.nosynth::first-line {
        font-weight: bold;
        font-synthesis-weight: none;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
