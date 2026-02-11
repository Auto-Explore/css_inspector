# css/css-conditional/at-supports-content-003.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-content-003.html"
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

		@supports (color: blue) {
			@keyframes green {
				from {
					background: green;
				}
				to {
					background: green;
				}
			}
		}
		@supports not (color: blue) {
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
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
