# css/css-conditional/at-supports-010.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-010.html"
}
```

## style[0]

```css

			div {
				background-color:red;
				height:100px;
				width:100px;
			}
			@supports ((yada: 2kg !trivial) or ((not (foo:bar)) and (((visibility:hidden))))) {
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
