# css/css-flexbox/flexbox_flex-N-N-auto-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_flex-N-N-auto-ref.html"
}
```

## style[0]

```css

div {
	background: blue;
	margin: 1em 0;
	border: 1px solid black;
	height: 8em;
	width: 40em;
}
span {
	background: yellow;
	margin: 1em 0;
	width: 10em;
	height: 6em;
	display: inline-block;
}
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
