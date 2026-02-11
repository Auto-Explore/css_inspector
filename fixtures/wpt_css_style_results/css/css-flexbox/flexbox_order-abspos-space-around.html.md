# css/css-flexbox/flexbox_order-abspos-space-around.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_order-abspos-space-around.html"
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

	display: flex;
	justify-content: space-around;
}
span {
	background: yellow;
	width: 5em;
	flex: 0 0 auto;
}
#test {
	position: absolute;
	right: 0;
	order: -1;
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
