# css/css-flexbox/flexbox_first-line.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_first-line.html"
}
```

## style[0]

```css

ul {
	background: #3366cc;
	padding: 0;
	list-style: none;
	width: 900px;

	display: flex;
	flex-wrap: wrap;
}
li {
	background: #ffcc00;
	margin: 2em;
	border: 1px solid black;
}
ul:first-child::first-line {
	justify-content: space-around;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
