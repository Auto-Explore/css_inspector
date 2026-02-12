# css/css-flexbox/flexbox_align-content-flexstart.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_align-content-flexstart.html"
}
```

## style[0]

```css

div {
	background: blue;
	margin: 1em 0;
	border: 1px solid black;
	height: 6em;
	width: 20em;

	display: flex;
	flex-wrap: wrap;
	align-content: flex-start;
}
span {
	background: white;
	margin: 0 1em;
	width: 8em;
	height: 2em;
	display: inline-block;

	flex: none;
}
span:nth-child(1) {background: yellow;}
span:nth-child(2) {background: pink;}
span:nth-child(3) {background: lightblue;}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
