# css/css-flexbox/flexbox_flex-natural-variable-auto-basis-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_flex-natural-variable-auto-basis-ref.html"
}
```

## style[0]

```css

div {
	border: 1px solid black;
	height: 8em;
	width: 32em;
}
span {
	width: 7em;
	height: 8em;
	float: left;
}
span:nth-child(2) {
	width: 11em;
}
span:nth-child(1) {background: yellow;}
span:nth-child(2) {background: pink;}
span:nth-child(3) {background: lightblue;}
span:nth-child(4) {background: grey;}
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
