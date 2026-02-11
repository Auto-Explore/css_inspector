# css/css-flexbox/css-flexbox-height-animation-stretch-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/css-flexbox-height-animation-stretch-ref.html"
}
```

## style[0]

```css

		@keyframes resize {
			0%   {height: 100px;}
			100% {height: 50px;}
		}
        .test {
            height: 100px;
            width: 200px;
            background-color: green;
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
