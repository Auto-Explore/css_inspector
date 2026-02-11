# css/css-conditional/at-media-002.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-media-002.html"
}
```

## style[0]

```css

		div {
			background: green;
			height: 25px;
			width: 100px;
		}
		@media not all {
			.test1 { background: red; }
		}
		@media unknown {
			.test2 { background: red; }
		}
		@media (unknown) {
			.test3 { background: red; }
		}
		@media not (unknown) {
			.test4 { background: red; }
		}
	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
