# css/css-color/t421-rgb-hex-parsing-f.xht

```json
{
  "format_version": 3,
  "file": "css/css-color/t421-rgb-hex-parsing-f.xht"
}
```

## style[0]

```css
<![CDATA[
			p { color: green; }
			p { color: #f; }
			p { color: #ff; }
			p { color: #ff000; }
			p { color: #ff00000; }
			p { color: #ff0000000; }
			p { color: #ff00000000; }
		]]>
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
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
