# css/css-flexbox/flexbox_margin-auto-overflow.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_margin-auto-overflow.html"
}
```

## style[0]

```css

div {
	background: blue;
	margin: 1em 0;
	border: 1px solid black;
	height: 8em;
	width: 32em;
	position: relative;

	display: flex;
}
span {
	background: white;
	margin: 1em auto;
	width: 30em;
	display: inline-block;
	flex: 0 0 auto;
}
span+span {background: yellow;}
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
