# css/css-conditional/at-supports-014.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-014.html"
}
```

## style[0]

```css

			div {
				background-color:red;
				height:100px;
				width:100px;
			}
			@supports (width: 0) {
				div { background-color:green; }
			}
			@supports not(foo: baz) {
				div { background-color:red; }
			}
		
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
