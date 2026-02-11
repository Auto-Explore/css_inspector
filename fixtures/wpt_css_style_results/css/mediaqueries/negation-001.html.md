# css/mediaqueries/negation-001.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/negation-001.html"
}
```

## style[0]

```css

			div {
				background-color: red;
				height: 20px;
				width: 100px;
			}
			@media not print {
				.test1 { background: green; }
			}
			@media not (monochrome) {
				.test2 { background: green; }
			}
			@media (not (monochrome)) {
				.test3 { background: green; }
			}
			@media not (not (color)) {
				.test4 { background: green; }
			}
			.test5 { background: green; }
			@media not ((unknown) or (monochrome)) {
				.test5 { background: red; }
			}
	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
