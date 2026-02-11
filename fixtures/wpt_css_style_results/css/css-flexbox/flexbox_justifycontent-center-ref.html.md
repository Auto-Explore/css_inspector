# css/css-flexbox/flexbox_justifycontent-center-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_justifycontent-center-ref.html"
}
```

## style[0]

```css

div {
	background: blue;
	margin: 1em 0;
	border: 1px solid black;
	height: 8em;
	width: 30em;
	position: relative;
}
span {
	background: yellow;
	margin: 1em 0;
	width: 6em;
	max-width: 6em;
	height: 6em;
	position: absolute;
	left: 4em;
	display: inline-block;
}
span:nth-child(2) {
	background: pink;
	left: 12em;
}
span:nth-child(3) {
	background: lightblue;
	left: 20em;
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
