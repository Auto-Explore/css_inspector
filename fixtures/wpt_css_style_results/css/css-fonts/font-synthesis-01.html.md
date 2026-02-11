# css/css-fonts/font-synthesis-01.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-synthesis-01.html"
}
```

## style[0]

```css

    @font-face {
				font-family: "test";
				src: url(/fonts/Lato-Medium.ttf);
			}
    @supports not (font-synthesis: none) {
        .test {color: red;}
    }
    .test {
				font-family: "test";
				font-size: 3em;
			}
    .nosynth {
        font-weight: bold;
        font-synthesis: none;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
