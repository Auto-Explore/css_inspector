# css/css-flexbox/flexbox_flex-initial-2.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_flex-initial-2.html"
}
```

## style[0]

```css

div {
	background: blue;
	margin: 1em 0;
	border: 1px solid black;
	height: 8em;
	width: 40em;

	display: flex;
}
span {
	background: white;
	margin: 1em;
	width: 5em;
	display: inline-block;
	flex: initial;
}
#flex span {
	flex: 0 1 auto;
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
