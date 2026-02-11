# css/css-fonts/font-synthesis-05.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-synthesis-05.html"
}
```

## style[0]

```css

    @font-face {
				font-family: "test";
				src: url(/fonts/Lato-Medium.ttf);
			}
    @supports not (font-synthesis: weight style) {
        .test p {color: red;}
    }
    .test {
				font-family: "test";
				font-size: 3em;
                font-synthesis: weight style;
                color: green;
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
