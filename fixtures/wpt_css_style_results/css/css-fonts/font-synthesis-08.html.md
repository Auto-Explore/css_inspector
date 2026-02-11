# css/css-fonts/font-synthesis-08.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-synthesis-08.html"
}
```

## style[0]

```css

    @font-face {
        font-family: "lato";
        src: url(/fonts/Lato-Medium.ttf);
    }
    .test {
        font-family: "lato";
        font-size: 3em;
        font-style: italic;
        font-weight: bold;
    }
    .auto {
        font-synthesis: weight style;
    }
    .none {
        font-synthesis: none;
    }
    .weight {
        font-synthesis: weight;
    }
    .style {
        font-synthesis: style;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “font-synthesis”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
