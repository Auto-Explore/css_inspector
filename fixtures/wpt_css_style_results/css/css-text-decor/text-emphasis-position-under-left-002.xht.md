# css/css-text-decor/text-emphasis-position-under-left-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-emphasis-position-under-left-002.xht"
}
```

## style[0]

```css
<![CDATA[
			.test
			{
				text-emphasis-style: filled circle;
				text-emphasis-position: under left;
			}
			.vertical
			{
				writing-mode: vertical-rl;
			}
			/* the CSS below is not part of the test */
			.displayarea
			{
				border: solid 1px gray;
				padding: 10px;
				width: 10em;
			}
			.floatright
			{
				float: right;
			}
			.ref {
				ruby-position: under;  /* to match emphasis position 'left' */
			}
			.parent
			{
				background: yellow;
				border: solid 1px gray;
				color: blue;
				font: 20px/1em ahem;
				height: 6em;
				width: 2em;
			}
		]]>
```

```json
{
  "errors": 4,
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
      "message": "Invalid value for property “text-emphasis-style”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-emphasis-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
