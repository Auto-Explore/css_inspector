# css/css-values/attr-length-valid.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-length-valid.html"
}
```

## style[0]

```css


			html, body { margin: 0px; padding: 0px; }

			html { background: white; overflow: hidden; }
			#outer { position: relative; background: green; }

			#outer { width: attr(data-test type(<length>)); height: 200px; }

	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
