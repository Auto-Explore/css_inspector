# css/css-flexbox/flexbox_align-content-spacebetween-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_align-content-spacebetween-ref.html"
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
	position: relative;
}
span {
	background: yellow;
	width: 8em;
	height: 2em;
	position: absolute;
	top: 0;
	left: 1em;
	display: inline-block;
}
span:nth-child(2) {
	background: pink;
	left: 11em;
}
span:nth-child(3) {
	background: lightblue;
	top: 4em;
	left: 1em;
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
