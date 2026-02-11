# css/css-flexbox/css-flexbox-height-animation-stretch.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/css-flexbox-height-animation-stretch.html"
}
```

## style[0]

```css

		@keyframes resize {
			0%   {height: 100px;}
			100% {height: 50px;}
		}
		.container {
			display: flex;
			width: 200px;
			background-color: red;

		}
		.item {
			background-color: green;
			width: 50px;

		}
		.content {
			height: 50px;
		}
		.bigger.content {
			height: 100px;
			animation: resize 500ms infinite alternate;
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
