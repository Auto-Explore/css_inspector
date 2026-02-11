# css/css-flexbox/flexbox_fbfc.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_fbfc.html"
}
```

## style[0]

```css

* {margin: 0;}
body {width: 600px;}
#float {
	background: #3366cc;
	width: 25%;
	float: left;
}
#flex {
	background: #ffcc00;
	width: 80%;
	display: flex;
}
div div {
	background: pink;
	margin: 2em;
	height: 4em;
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
