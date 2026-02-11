# css/css-flexbox/flexbox_rtl-flow-reverse.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_rtl-flow-reverse.html"
}
```

## style[0]

```css

div {
	background: blue;
	margin: 1em 0;
	border: 1px solid black;
	width: 20em;
	height: 8em;
	direction: rtl;

	display: flex;
	flex-flow: column wrap-reverse;
}
span {
	background: white;
	margin: 1em;
	width: 8em;
	display: inline-block;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex-flow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
