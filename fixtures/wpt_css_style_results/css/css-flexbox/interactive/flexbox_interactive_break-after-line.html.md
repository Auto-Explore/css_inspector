# css/css-flexbox/interactive/flexbox_interactive_break-after-line.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/interactive/flexbox_interactive_break-after-line.html"
}
```

## style[0]

```css

* {widows: 1; orphans: 1;}
div {
	width: 20em;

	display: flex;
	flex-wrap: wrap;
}
p {
	background: white;
	margin: 0;
	width: 100%;
	height: 2em;

	flex: 1 0 auto;
}
p+p {background: red;}
@media projection, print {
	p:first-child {
		break-after: always;
		background: yellow;
	}
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
