# css/css-flexbox/flexbox_align-items-stretch.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_align-items-stretch.html"
}
```

## style[0]

```css

div {
	background: blue;
	margin: 1em 0;
	border: 1px solid black;
	height: 6em;
	width: 30em;

	display: flex;
	align-items: stretch;
}
span {
	background: white;
	margin: 0 1em;
	width: 8em;
	display: inline-block;

	flex: none;
}
span:nth-child(1) {background: yellow;}
span:nth-child(2) {
	background: pink;
	margin-top: 3em;
}
span:nth-child(3) {
	background: lightblue;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
