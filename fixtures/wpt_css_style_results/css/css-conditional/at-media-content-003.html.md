# css/css-conditional/at-media-content-003.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-media-content-003.html"
}
```

## style[0]

```css

		div {
			background: red;
			width: 100px;
			height: 100px;
			animation: green 4s both;
		}

		@media all {
			@keyframes green {
				from {
					background: green;
				}
				to {
					background: green;
				}
			}
		}
		@media not all {
			@keyframes green {
				from {
					background: red;
				}
				to {
					background: red;
				}
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
