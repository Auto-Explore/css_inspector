# css/css-conditional/at-supports-015.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-015.html"
}
```

## style[0]

```css

			div {
				background-color:red;
				height:100px;
				width:100px;
			}
			@supports (not (foo: baz !unrelated $ ,/[&])) {
				div { background-color:green; }
			}
		
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
