# css/css-flexbox/flexbox_flex-formatting-interop.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_flex-formatting-interop.html"
}
```

## style[0]

```css

* {margin: 0;}
.float {
	background: lightblue;
	width: 200px;
	float: left;
}
#flex {
	background: #ffcc00;
	margin-left: -200px;
	width: 200px;

	display: flex;
}
div div {
	border: 2px solid transparent;
	margin: 0 2em 2em;
	flex: none;
}
p {
	clear: both;
	margin: 2em 0;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
