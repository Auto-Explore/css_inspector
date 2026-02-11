# css/css-multicol/multicol-rule-shorthand-2.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-shorthand-2.xht"
}
```

## style[0]

```css
<![CDATA[
body {
	margin: 1em;
}
body>div {
	font-family: Ahem;
	font-size: 1em;
	line-height: 1em;
	color: black;
	background: yellow;
	margin: 1em 0;
	border: 1em solid gray;
	width: 15em;

	column-count: 4;
	column-gap: 1em;
	column-rule: solid blue 1em;
	column-rule: normal red 1em; /* invalid: 'normal' is not a 'border-style' */
	column: normal red 1em; /* invalid: 'column' is not a valid property name; 'normal' can only apply to 'column-gap' */
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
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “column”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
