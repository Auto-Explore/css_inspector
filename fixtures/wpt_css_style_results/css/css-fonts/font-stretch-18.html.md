# css/css-fonts/font-stretch-18.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-stretch-18.html"
}
```

## style[0]

```css

    @font-face {
				font-family: "test";
				src: url(support/fonts/pass.woff) format("woff");
                font-stretch: extra-condensed;
			}
    @font-face {
				font-family: "test";
				src: url(support/fonts/fail.woff) format("woff");
                font-stretch: expanded;
			}
    .test {
				font-family: "test";
				font-size: 6em;
                font-stretch: condensed;
			}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
