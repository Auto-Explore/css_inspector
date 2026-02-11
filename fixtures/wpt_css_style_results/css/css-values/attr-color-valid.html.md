# css/css-values/attr-color-valid.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-color-valid.html"
}
```

## style[0]

```css


			html, body { margin: 0px; padding: 0px; }

			html { background: white; overflow: hidden; }
			#outer { position: relative; background: red; width: 200px; height: 200px; }

			#outer { background: attr(data-test type(<color>)); }

	
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
