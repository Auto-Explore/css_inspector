# css/css-ui/canvas-cursor-002.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/canvas-cursor-002.html"
}
```

## style[0]

```css

:root { padding:0; background:blue;}
body {
	margin:0;
	cursor: url(support/red.ico), pointer;
}
p {
	background: white;
	cursor: default;
	position:absolute;
	/* the p is taken out of flow to make the html and body elements collapse to nothing,
	leaving the whole background area empty of any element,
	as that's what we want to hover over */ }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
