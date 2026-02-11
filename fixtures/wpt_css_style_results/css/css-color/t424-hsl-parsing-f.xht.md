# css/css-color/t424-hsl-parsing-f.xht

```json
{
  "format_version": 3,
  "file": "css/css-color/t424-hsl-parsing-f.xht"
}
```

## style[0]

```css
<![CDATA[
		html, body { background: white; }
		p { color: hsl(120, 100%, 25%); }
		p { color: hsl(0, 255, 128); }
		p { color: hsl(0%, 100%, 50%); }
		p { color: hsl(0px, 100%, 50%); }
		]]>
```

```json
{
  "errors": 3,
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
    }
  ],
  "warnings": 0
}
```
