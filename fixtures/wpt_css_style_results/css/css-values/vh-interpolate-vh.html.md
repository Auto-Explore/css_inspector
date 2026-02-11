# css/css-values/vh-interpolate-vh.html

```json
{
  "format_version": 3,
  "file": "css/css-values/vh-interpolate-vh.html"
}
```

## style[0]

```css


			@keyframes anim {
				from { width: 75vw; height: 75vh; }
				to   { width: 125vw; height: 125vh; }
			}

			html, body { margin: 0px; padding: 0px; }

			html { background: red; overflow: hidden; }
			#outer { position: relative; background: green; }
			#outer { animation: anim 2000000s; animation-delay: -1000000s; }

	
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
