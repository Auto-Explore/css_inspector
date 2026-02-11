# css/css-values/attr-length-invalid-cast.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-length-invalid-cast.html"
}
```

## style[0]

```css


			html, body { margin: 0px; padding: 0px; }

			html { background: white; overflow: hidden; }
			#outer { position: relative; background: green; }

			#outer { width: attr(data-test type(<length>), 200px); height: 200px; }

	
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
