# css/css-flexbox/interactive/flexbox_interactive_flex-shrink-transitions.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/interactive/flexbox_interactive_flex-shrink-transitions.html"
}
```

## style[0]

```css

div {
	width: 12em;
	height: 8em;

	display: flex;
}
span {
	background: yellow;
	width: 8em;
	flex: 0 1 auto;
}
#test, #ref {
	background: #3366cc;
}
div:hover #test {
	transition: flex-shrink 4s;
	flex-shrink: 2;
}
p~div {
	margin-right: 1em;
	float: left;
	display: block;
}
p~div span {
	width: 3em;
	height: 8em;
	float: left;
}
p~div~div span {
	width: 4em;
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
