# css/mediaqueries/mq-calc-002.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-calc-002.html"
}
```

## style[0]

```css

			:root { font-size: 30000px; }
			p { font-size: 16px; }
			div {
				width: 100px;
				height: 100px;
				background-color: red;
			}
			@media (min-width: calc(1em)){
				div { background-color: green; }
			}
		
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

			@media (min-width: calc(1em)){
				div { background-color: green; }
			}
		
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
