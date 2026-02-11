# css/mediaqueries/mq-calc-007.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-calc-007.html"
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
	@media (min-width: calc(1px + 1rem)) {
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
