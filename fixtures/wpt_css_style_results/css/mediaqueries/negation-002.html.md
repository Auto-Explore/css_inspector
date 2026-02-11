# css/mediaqueries/negation-002.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/negation-002.html"
}
```

## style[0]

```css

			div {
				background-color: red;
				height: 25px;
				width: 100px;
			}

			@media ((color) and (color)) {
				/* Only green when the browser supports the general syntax */
				div {
					background-color: green;
				}
			}

			@media (not (monochrome) and (color)) {
				.test1 { background: red; }
			}
			@media (not (monochrome) or (color)) {
				.test2 { background: red; }
			}
			@media ((color) and not (monochrome)) {
				.test3 { background: red; }
			}
			@media ((color) or not (monochrome)) {
				.test4 { background: red; }
			}
	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
