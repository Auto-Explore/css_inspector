# css/css-flexbox/flexbox_rtl-order-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_rtl-order-ref.html"
}
```

## style[0]

```css

div {
	background: blue;
	padding: 1em 0 0;
	margin: 1em 0;
	border: 1px solid black;
	width: 20em;
	height: 7em;
}
span {
	text-align: right;
	background: white;
	margin: 1em;
	width: 8em;
	height: 1.5em;
	float: left;
}
.one {
	background: pink;
}
.two {
	background: yellow;
}
.three {
	background: black;
	color: white;
}
.four {
	background: fuchsia;
	color: white;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
