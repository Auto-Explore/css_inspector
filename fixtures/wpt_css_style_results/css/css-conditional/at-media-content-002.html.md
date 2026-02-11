# css/css-conditional/at-media-content-002.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-media-content-002.html"
}
```

## style[0]

```css

		div {
			background: red;
			color: green;
			font: 100px/1 Blockify, sans-serif;
			width: 100px;
		}

		@media all {
			@font-face {
				font-family: 'Blockify';
				src: url('/fonts/Ahem.ttf');
			}
		}
		@media not all {
			@font-face {
				font-family: 'Blockify';
				src: local('Arial'), url('/fonts/fail.woff');
			}
		}
	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
