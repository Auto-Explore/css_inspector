# css/css-conditional/at-supports-001.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-001.html"
}
```

## style[0]

```css

		div {
			background-color:red;
			height:50px;
			width:100px;
		}
		@supports (margin: 0) {
			.test1 { background:green; }
			.test2 { background: red; }
		}
		.test2 { background: green; }
	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
