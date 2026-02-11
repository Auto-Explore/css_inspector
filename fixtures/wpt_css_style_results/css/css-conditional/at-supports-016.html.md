# css/css-conditional/at-supports-016.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-016.html"
}
```

## style[0]

```css

			div{
				background-color:red;
				height:100px;
				width:100px;
			}
			@supports (margin:0) { div { background-color:green; } }
			@supports ((margin: 0) and (display: inline) or (width:1em)) {
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
