# css/css-flexbox/flexbox_align-items-stretch-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_align-items-stretch-ref.html"
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
	margin: 0;
	width: 8em;
	height: 6em;
	position: absolute;
	top: 0;
	left: 1em;
	display: inline-block;
}
span:nth-child(2) {
	background: pink;
	height: 3em;
	top: 3em;
	left: 11em;
}
span:nth-child(3) {
	background: lightblue;
	height: 6em;
	top: 0;
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
