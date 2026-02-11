# css/css-text-decor/text-emphasis-position-over-left-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-emphasis-position-over-left-001.xht"
}
```

## style[0]

```css
<![CDATA[
			.test
			{
				text-emphasis-style: filled circle;
				text-emphasis-position: over left;
			}
			/* the CSS below is not part of the test */
			.parent
			{
				background: yellow;
				border: solid 1px gray;
				color: blue;
				font: 20px/1em ahem;
				width: 6em;
			}
			ruby {
			    ruby-position: over;
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
