# css/css-flexbox/interactive/flexbox_interactive_flex-grow-transitions.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/interactive/flexbox_interactive_flex-grow-transitions.html"
}
```

## style[0]

```css

div {
	width: 20em;
	height: 8em;

	display: flex;
}
span {
	background: yellow;
	width: 8em;
	flex: 1 0 0%;
}
#test, .ref {
	background: #3366cc;
}
div:hover #test {
	transition: flex-grow 4s;
	flex-grow: 2;
}
p~div {
	margin-right: 1em;
	float: left;
	display: block;
}
p~div span {
	width: 5em;
	height: 8em;
	float: left;
}
p~div~div span {
	width: 4em;
}
p~div~div span.ref {
	width: 8em;
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
