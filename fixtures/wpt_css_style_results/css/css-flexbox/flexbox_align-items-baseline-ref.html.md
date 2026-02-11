# css/css-flexbox/flexbox_align-items-baseline-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_align-items-baseline-ref.html"
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
	background: white;
	width: 8em;
	height: 2em;
	display: block;
	position: absolute;
	top: 1em;
	left: 1em;
}
span:nth-child(1) {background: yellow;}
span:nth-child(2) {
	background: pink;
	height: 3em;
	left: 11em;
}
span:nth-child(3) {
	background: lightblue;
	height: 4em;
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
