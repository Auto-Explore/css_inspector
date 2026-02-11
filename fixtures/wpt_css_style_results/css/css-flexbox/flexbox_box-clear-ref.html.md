# css/css-flexbox/flexbox_box-clear-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_box-clear-ref.html"
}
```

## style[0]

```css

#float {
	background: #3366cc;
	padding: 1em;
	float: left;

}
#flex {
	background: #ffcc00;
	padding: 2em;
	clear: both;
}
div div {
	background: pink;
	height: 4em;
	display: inline-block;
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
