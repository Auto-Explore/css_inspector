# css/css-conditional/at-media-content-004.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-media-content-004.html"
}
```

## style[0]

```css

		div {
			background: red;
			color: green;
			font: 100px/1 Ahem, sans-serif;
			width: 100px;
			height: 100px;
		}
		div::before {
			content: counter(test, x);
		}

		@media all {
			@counter-style x {
				system: cyclic;
				symbols: 'X';
			}
		}
		@media not all {
			@counter-style x {
				system: cyclic;
				symbols: ' ';
			}
		}
	
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
