# css/css-flexbox/interactive/flexbox_interactive_break-after-column-item.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/interactive/flexbox_interactive_break-after-column-item.html"
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
	margin: 0;
	height: 2em;

	flex: 1;
}
@media projection, print {
	p:first-child {
		break-after: always;
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
