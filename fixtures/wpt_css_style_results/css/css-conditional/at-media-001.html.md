# css/css-conditional/at-media-001.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-media-001.html"
}
```

## style[0]

```css

		div {
			background: red;
			height: 25px;
			width: 100px;
		}

		@media all {
			.test1 { background: green; }
			.test2 { background: red; }
		}
		.test2 { background: green; }

		@media not unknown {
			.test3 { background: green; }
			.test4 { background: red; }
		}
		.test4 { background: green; }
	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
