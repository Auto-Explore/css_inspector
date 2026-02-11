# css/css-conditional/at-supports-018.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-018.html"
}
```

## style[0]

```css

			div {
				background:red;
				height:100px;
				width:100px;
			}
			@supports (not (width:compute( 2px + 2px ))) {
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
