# css/css-values/vh-interpolate-pct.html

```json
{
  "format_version": 3,
  "file": "css/css-values/vh-interpolate-pct.html"
}
```

## style[0]

```css


			@keyframes anim {
				from { width: 0%; height: 0%; }
				to   { width: 200vw; height: 200vh; }
			}

			html, body { margin: 0px; padding: 0px; height: 100%; }

			html { background: red; overflow: hidden; }
			#outer { position: relative; background: green; }
			#outer { animation: anim 2000000s; animation-delay: -1000000s; }

	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
