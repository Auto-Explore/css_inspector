# css/css-multicol/multicol-inherit-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-inherit-003.xht"
}
```

## style[0]

```css
<![CDATA[
body {
	width: 75em;
}
body>div {
	height: 2em;
	column-width: 8em;
}
div {
	font-family: Ahem;
	font-size: 1em;
	line-height: 1em;
	color: black;
	background: yellow;
	orphans: 1;
	widows: 1;
}
div>div {
	margin: 0 1em 1em;
}
div+div {
	color: blue;
}
div+div+div {
	color: pink;
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
