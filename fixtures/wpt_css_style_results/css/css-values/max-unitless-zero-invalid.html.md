# css/css-values/max-unitless-zero-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-values/max-unitless-zero-invalid.html"
}
```

## style[0]

```css

			html, body { margin: 0px; padding: 0px; }

			html { background: red; overflow: hidden; }
			#outer { position: absolute; top: 0px; left: 0px; background: green; width: 100%; }

			#outer {
				/* Assert that min() is supported */
				height: min(100%);

	 			/* The min() expression (thus the declaration) should be invalid */
				height: min(0, 100%);
			}
	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
