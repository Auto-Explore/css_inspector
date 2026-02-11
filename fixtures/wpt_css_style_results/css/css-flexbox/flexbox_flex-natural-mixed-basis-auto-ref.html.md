# css/css-flexbox/flexbox_flex-natural-mixed-basis-auto-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_flex-natural-mixed-basis-auto-ref.html"
}
```

## style[0]

```css

div {
	font-family: ahem;
	background: blue;
	height: 8em;
	width: 35em;
}
span {
	font-family: ahem;
	height: 8em;
	display: inline-block;
}
span:nth-child(1) {
	background: yellow;
	width: 3em;
}
span:nth-child(2) {
	background: pink;
	width: 6em;
}
span:nth-child(3) {
	background: lightblue;
	width: 8em;
}
span:nth-child(4) {
	background: grey;
	width: 18em;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
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
