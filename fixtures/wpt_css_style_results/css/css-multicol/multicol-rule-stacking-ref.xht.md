# css/css-multicol/multicol-rule-stacking-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-stacking-ref.xht"
}
```

## style[0]

```css
<![CDATA[
body {
	margin: 0;
}
body>div {
	font-family: Ahem;
	font-size: 1em;
	line-height: 1em;
	color: black;
	background: blue;
	margin: 1em;
	border: 1em solid gray;
	width: 14em;
	height: 2em;
	position: relative;
}
span {
	margin-left: 7.5em;
	width: 2em;
	height: 2em;
	display: inline-block;
}
div div {
	background: blue;
	position: absolute;
	height: 2em;
	width: 2em;
	top: 0;
	left: -2em;
}
div div+div {
	left: 13em;
	width: 11.75em;
}
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
