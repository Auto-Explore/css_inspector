# css/css-flexbox/flexbox_flex-basis.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_flex-basis.html"
}
```

## style[0]

```css

div {
	background: blue;
	margin: 1em 0;
	border: 1px solid black;
	height: 8em;
	width: 8em;

	display: flex;
}
span {
	background: white;
	margin: 1em 0;
	width: 5em;
	display: inline-block;

	flex-grow: 1;
	flex-shrink: 0;
	flex-basis: 100%;
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
