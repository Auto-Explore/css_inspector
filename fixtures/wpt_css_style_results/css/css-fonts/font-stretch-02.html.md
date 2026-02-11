# css/css-fonts/font-stretch-02.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-stretch-02.html"
}
```

## style[0]

```css

    @font-face {
				font-family: "test";
				src: url(support/fonts/fail.woff) format("woff");
                font-stretch: normal;
			}
    @font-face {
				font-family: "test";
				src: url(support/fonts/pass.woff) format("woff");
                font-stretch: ultra-expanded;
			}
    .test {
				font-family: "test";
				font-size: 6em;
                font-stretch: ultra-expanded;
			}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
