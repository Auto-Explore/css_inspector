# css/css-flexbox/interactive/flexbox_interactive_break-natural.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/interactive/flexbox_interactive_break-natural.html"
}
```

## style[0]

```css

* {
	widows: 1;
	orphans: 1;
	margin: 0;
	height: 100%;
}
body {
	display: flex;
	flex-direction: column;
}
div {
	height: 75%;
	flex: 0 0  auto;
}
@media print {
	div {
		break-inside: avoid;
	}
	div+div {
		border-top: 1em solid red;
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
