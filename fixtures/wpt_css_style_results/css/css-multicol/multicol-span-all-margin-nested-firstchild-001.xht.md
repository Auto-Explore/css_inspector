# css/css-multicol/multicol-span-all-margin-nested-firstchild-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-span-all-margin-nested-firstchild-001.xht"
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
	color: navy;
	background: yellow;
	margin: 1em;
	border: 1em solid gray;
	width: 8em;
	float: left;
	orphans: 1;
	widows: 1;
	overflow: hidden;

	column-count: 4;
	column-gap: 0;
}
span, h6 {
	font-family: Ahem;
	font-size: 1em;
	font-weight: normal;
	line-height: 1em;
	color: black;
	background: black;
	padding: 0;
	margin: 0;
	display: block;
}
h6 {
	margin: 1em 0;
	width: 11em;
	column-span: all;
}
span {
	color: pink;
	background: pink;
	margin: 2em 0;
}
h6~h6 {
	color: black;
	background: black;
	height: 1em;
}
h6~h6>span {
	background: pink;
	color: pink;
	margin: -2em 0 0;
}
]]>
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
