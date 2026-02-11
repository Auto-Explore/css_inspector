# css/mediaqueries/mq-invalid-media-type-layer-001.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-invalid-media-type-layer-001.html"
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
			@media not layer {
				div { background-color: red; }
			}
			@media layer {
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
