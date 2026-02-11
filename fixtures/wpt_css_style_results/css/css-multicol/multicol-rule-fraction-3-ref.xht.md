# css/css-multicol/multicol-rule-fraction-3-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-fraction-3-ref.xht"
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
	height: 2em;
	position: relative;
}
div div {
	width: 2em;
	height: 2em;
	background: black;
	position: absolute;
	left: 0;
	top: 0;
}
.a {
	background: blue;
	width: 1em;
}
#a1 {left: 2.4em;}
#a2 {left: 3.75em;}
#a3 {left: 6.15em;}
#a4 {left: 7.5em;}
#a5 {left: 9.9em;}
#a6 {left: 11.25em;}
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
