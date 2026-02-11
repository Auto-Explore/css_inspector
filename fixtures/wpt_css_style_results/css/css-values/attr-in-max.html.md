# css/css-values/attr-in-max.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-in-max.html"
}
```

## style[0]

```css

			html, body { margin: 0px; padding: 0px; }

			html { background: white; overflow: hidden; }
			#outer { position: relative; background: green; }

			#outer { width: max(attr(data-test type(<length>))); height: 200px; }
	
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
