# css/css-flexbox/flexbox_wrap-long-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_wrap-long-ref.html"
}
```

## style[0]

```css

div {
	background: blue;
	margin: 1em 0;
	border: 1px solid black;
	width: 20em;
}
span {
	background: white;
	margin: 1em;
	width: 8em;
	display: block;
	float: left;
}
span:first-child {
	width: 18em;
}
div::after {
	content: "";
	clear: both;
	display: block;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
