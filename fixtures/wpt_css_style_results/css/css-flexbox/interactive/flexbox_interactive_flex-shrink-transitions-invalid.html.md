# css/css-flexbox/interactive/flexbox_interactive_flex-shrink-transitions-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/interactive/flexbox_interactive_flex-shrink-transitions-invalid.html"
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
	flex: 0 1 auto;
}
#test, .ref {
	background: #3366cc;
}
div:hover #test {
	transition: flex-shrink 4s;
	flex-shrink: 0;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
