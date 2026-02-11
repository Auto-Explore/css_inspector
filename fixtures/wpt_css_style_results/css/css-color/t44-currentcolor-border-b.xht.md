# css/css-color/t44-currentcolor-border-b.xht

```json
{
  "format_version": 3,
  "file": "css/css-color/t44-currentcolor-border-b.xht"
}
```

## style[0]

```css
<![CDATA[
		#one {
			color: green;
			border: medium solid red;
			border: medium solid currentColor;
		}
		#two {
			color: green;
			border: medium solid red;
			border-color: currentcolor;
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
