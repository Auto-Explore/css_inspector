# css/css-conditional/at-supports-009.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-009.html"
}
```

## style[0]

```css

			div {
				background-color:red;
				height:100px;
				width:100px;
			}
			@supports (not (yadayada: x, calc( 2/3 ))) {
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
