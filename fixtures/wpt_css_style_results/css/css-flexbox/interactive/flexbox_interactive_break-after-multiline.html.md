# css/css-flexbox/interactive/flexbox_interactive_break-after-multiline.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/interactive/flexbox_interactive_break-after-multiline.html"
}
```

## style[0]

```css

* {widows: 1; orphans: 1;}
div {
	border: 1px solid white;
	width: 20em;

	display: flex;
	flex-wrap: wrap;
}
p {
	background: yellow;
	margin: 0;
	height: 2em;

	flex: 1 0 10em;
}
@media projection, print {
	#test {break-after: always;}
	#test~p {background: red;}
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
