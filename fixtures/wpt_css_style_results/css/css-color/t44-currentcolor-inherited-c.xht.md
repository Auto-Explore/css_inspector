# css/css-color/t44-currentcolor-inherited-c.xht

```json
{
  "format_version": 3,
  "file": "css/css-color/t44-currentcolor-inherited-c.xht"
}
```

## style[0]

```css
<![CDATA[
		#one {
			color: red;
			background: currentColor;
		}
		#two {
			color: green;
			/* inherit currentColor, which then is green */
			background: inherit;
		}
		#three {
			color: white;
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
