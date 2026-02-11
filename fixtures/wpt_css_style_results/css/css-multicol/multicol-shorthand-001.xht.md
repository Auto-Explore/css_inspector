# css/css-multicol/multicol-shorthand-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-shorthand-001.xht"
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
	width: 15em;
	orphans: 1;
	widows: 1;

	column-count: 4;
	column-gap: 1em;
	column-rule: solid blue 1em;
	column: normal red 1em; /* column is not a valid property */
	columns: normal red 1em; /* columns is a shorthand for colun-width and column-count */
}
span {
	background: blue;
	position: absolute;
	top: 0;
	left: 3em;
	height: 2em;
	width: 1em;
}
span+span {
	left: 7em;
}
span+span+span {
	left: 11em;
}
]]>
```

```json
{
  "errors": 5,
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
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
