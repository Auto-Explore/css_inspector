# css/css-flexbox/flexbox_flex-0-0-Npercent-shrink.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_flex-0-0-Npercent-shrink.html"
}
```

## style[0]

```css

div {
	background: blue;
	margin: 1em 0;
	border: 1px solid black;
	height: 8em;
	width: 12em;

	display: flex;
}
span {
	background: white;
	margin: 1em 0;
	width: 5em;
	display: inline-block;

	flex: 0 0 50%;
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
