# css/css-fonts/font-synthesis-04.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-synthesis-04.html"
}
```

## style[0]

```css

    @font-face {
				font-family: "test";
				src: url(/fonts/Lato-Medium.ttf);
			}
    @supports not (font-synthesis: weight) {
        .test {color: red;}
    }
    .test {
				font-family: "test";
				font-size: 3em;
			}
    .nosynth {
        font-style: italic;
        font-synthesis: weight;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
