# css/css-flexbox/flexbox_order-abspos-space-around-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_order-abspos-space-around-ref.html"
}
```

## style[0]

```css

div {
	background: blue;
	border: 1px solid black;
	width: 27em;
	height: 8em;
	position: relative;
}
span {
	background: yellow;
	width: 5em;
	height: 8em;
	position: absolute;
	top: 0;
	left: 2em;
	display: block;
}
span+span {left: 11em;}
span+span+span {left: 20em;}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
