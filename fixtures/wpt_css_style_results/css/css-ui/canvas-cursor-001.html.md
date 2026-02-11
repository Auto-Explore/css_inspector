# css/css-ui/canvas-cursor-001.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/canvas-cursor-001.html"
}
```

## style[0]

```css

:root {
	padding:0;
	background:blue;
	cursor: url(support/green.ico), pointer;
}
body {
	margin:0;
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
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “cursor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
