# css/css-conditional/at-supports-012.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-012.html"
}
```

## style[0]

```css

			div {
				background-color:red;
				height:100px;
				width:100px;
			}
			@supports ((margin: 0) and (background: blue) and (padding:inherit)) {
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
