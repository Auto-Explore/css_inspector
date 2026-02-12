# css/css-multicol/multicol-rule-fraction-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-fraction-003.xht"
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
	background: yellow;
	margin: 1em;
	border: 1em solid gray;
	width: 14em;
	position: relative;
	orphans: 1;
	widows: 1;

	column-count: 4;
	column-gap: 1em;
}
span {
	background: blue;
	position: absolute;
	top: 0;
	left: 2.4em;
	height: 2em;
	width: 1em;
}
span+span {
	left: 6.15em;
}
span+span+span {
	left: 9.9em;
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
