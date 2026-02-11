# css/css-flexbox/flexbox_align-self-auto-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_align-self-auto-ref.html"
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
	position: relative;
}
span {
	background: yellow;
	width: 8em;
	position: absolute;
	bottom: 0;
	left: 1em;
	display: inline-block;
}
span:nth-child(2) {
	background: pink;
	left: 11em;
}
span:nth-child(3) {
	background: lightblue;
	left: 21em;
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
