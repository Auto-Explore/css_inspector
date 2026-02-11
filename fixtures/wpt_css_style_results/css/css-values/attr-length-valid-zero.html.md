# css/css-values/attr-length-valid-zero.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-length-valid-zero.html"
}
```

## style[0]

```css


			html, body { margin: 0px; padding: 0px; }

			html { background: white; overflow: hidden; }
			#outer { position: relative; background: green; }
			#outer2 { background: red; }

			#outer { width: 200px; height: 200px; }
			#outer2 { width: 200px; width: attr(data-test type(<length>), 0); height: 200px; }

	
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
