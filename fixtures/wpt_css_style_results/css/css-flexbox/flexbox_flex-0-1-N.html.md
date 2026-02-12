# css/css-flexbox/flexbox_flex-0-1-N.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_flex-0-1-N.html"
}
```

## style[0]

```css

div {
	background: blue;
	margin: 1em 0;
	border: 1px solid black;
	height: 8em;
	width: 40em;

	display: flex;
}
span {
	background: white;
	margin: 1em 0;
	width: 5em;
	display: inline-block;

	flex: 0 1 4em;
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
