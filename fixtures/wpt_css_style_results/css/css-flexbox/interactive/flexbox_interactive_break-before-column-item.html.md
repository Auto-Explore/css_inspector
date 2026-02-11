# css/css-flexbox/interactive/flexbox_interactive_break-before-column-item.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/interactive/flexbox_interactive_break-before-column-item.html"
}
```

## style[0]

```css

* {widows: 1; orphans: 1;}
div {
	border: 1px solid white;
	width: 20em;

	display: flex;
	flex-direction: column;
}
p {
	background: white;
	margin: 0;
	height: 2em;

	flex: 1;
}
@media projection, print {
	p:last-child {
		break-before: always;
	}
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
