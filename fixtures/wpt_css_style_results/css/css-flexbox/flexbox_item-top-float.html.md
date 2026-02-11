# css/css-flexbox/flexbox_item-top-float.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_item-top-float.html"
}
```

## style[0]

```css

div {
	background: black;
	margin: 1em;
	width: 400px;

	display: flex;
}
p {
	background: #3366cc;
	margin: 2em;
	width: 2em;
	height: 2em;
	float: -o-top-corner;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “float”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
