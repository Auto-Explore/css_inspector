# css/css-flexbox/flexbox_justifycontent-spacearound-negative.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_justifycontent-spacearound-negative.html"
}
```

## style[0]

```css

div {
	background: blue;
	margin: 1em 0;
	border: 1px solid black;
	height: 8em;
	width: 17em;

	display: flex;
	justify-content: space-around;
}
span {
	background: white;
	margin: 1em;
	width: 4em;
	display: inline-block;

	flex: 0 0 auto;
}
span:nth-child(1) {background: yellow;}
span:nth-child(2) {background: pink;}
span:nth-child(3) {background: lightblue;}
```

```json
{
  "errors": 3,
  "messages": [
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
    }
  ],
  "warnings": 0
}
```
