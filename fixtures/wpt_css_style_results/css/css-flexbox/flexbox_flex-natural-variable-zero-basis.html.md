# css/css-flexbox/flexbox_flex-natural-variable-zero-basis.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_flex-natural-variable-zero-basis.html"
}
```

## style[0]

```css

div {
	border: 1px solid black;
	height: 8em;
	width: 36em;

	display: flex;
}
span {
	width: 6em;
	display: block;

	flex: 1 1 0%;
}
span:nth-child(2) {
	flex: 3 1 0%;
}
span:nth-child(1) {background: yellow;}
span:nth-child(2) {background: pink;}
span:nth-child(3) {background: lightblue;}
span:nth-child(4) {background: grey;}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
