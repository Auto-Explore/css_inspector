# css/css-fonts/font-synthesis-02.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-synthesis-02.html"
}
```

## style[0]

```css

    @font-face {
				font-family: "test";
				src: url(/fonts/Lato-Medium.ttf);
			}
    @supports not (font-synthesis: style) {
        .test {color: red;}
    }
    .test {
				font-family: "test";
				font-size: 3em;
			}
    .nosynth {
        font-weight: bold;
        font-synthesis: style;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
