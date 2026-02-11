# css/css-multicol/multicol-fill-auto-002-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-fill-auto-002-ref.xht"
}
```

## style[0]

```css
<![CDATA[
body>div {
	font-family: Ahem;
	font-size: 1.25em;
	line-height: 1em;
	color: green;
	height: 3em;
	width: 2em;
	orphans: 1;
	widows: 1;
	position: relative;
	margin: 1em;
}
div.col {
	column-count: 2;
	column-fill: auto;
	column-gap: 0;
}
div.red {
	background: red; position: absolute; z-index: -1;
}
]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
