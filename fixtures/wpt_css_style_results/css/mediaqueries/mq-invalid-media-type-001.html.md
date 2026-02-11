# css/mediaqueries/mq-invalid-media-type-001.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-invalid-media-type-001.html"
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
			@media not and {
				div { background-color: red; }
			}
			@media and {
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
