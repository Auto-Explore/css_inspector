# css/css-flexbox/flexbox_flex-natural-mixed-basis-auto.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_flex-natural-mixed-basis-auto.html"
}
```

## style[0]

```css

div {
	font-family: ahem;
	background: blue;
	height: 8em;
	width: 35em;

	display: flex;
}
span {
	font-family: ahem;
	background: white;

	flex: 1 1 auto;
}
span:nth-child(1) {
	flex: 1 1 0%;
}
span:nth-child(1) {background: yellow;}
span:nth-child(2) {background: pink;}
span:nth-child(3) {background: lightblue;}
span:nth-child(4) {background: grey;}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    },
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
