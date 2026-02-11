# css/css-flexbox/flexbox_justifycontent-spacebetween-only.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_justifycontent-spacebetween-only.html"
}
```

## style[0]

```css

div {
	background: blue;
	margin: 1em 0;
	border: 1px solid black;
	height: 8em;
	width: 30em;

	display: flex;
	justify-content: space-between;
}
span {
	background: white;
	margin: 1em;
	width: 5em;
	max-width: 6em;
	display: inline-block;

	flex: 1 0 0%;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
