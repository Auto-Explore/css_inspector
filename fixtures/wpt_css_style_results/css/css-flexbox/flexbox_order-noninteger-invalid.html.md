# css/css-flexbox/flexbox_order-noninteger-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_order-noninteger-invalid.html"
}
```

## style[0]

```css

div {
	width: 200px;
	height: 4em;
	display: flex;
}
span {
	background: red;
	order: 1.5;
	flex: 1 0 0%;
}
span+span {
	background: white;
	order: 0;
}
/* irrelevant */
* {margin: 0;}
h1 {
	width: 100px;
	height: 4em;
	background: white;
	position: absolute;
	top: 0;
	left: 0;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
