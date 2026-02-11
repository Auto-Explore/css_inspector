# css/mediaqueries/mq-invalid-media-type-002.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-invalid-media-type-002.html"
}
```

## style[0]

```css

			div {
				width: 100px;
				height: 100px;
			}
			@media all {
				div { background-color: green; }
			}
			@media not or {
				div { background-color: red; }
			}
			@media or {
				div { background-color: red; }
			}
	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
