# css/css-flexbox/flexbox_columns-flexitems-2.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_columns-flexitems-2.html"
}
```

## style[0]

```css

div {
	background: blue;

	display: flex;
	justify-content: space-around;
}
p {
	font-family: ahem;
	background: yellow;
	column-rule: 1em solid lime;
	columns: 2;
	margin: 0;
	width: 200px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
