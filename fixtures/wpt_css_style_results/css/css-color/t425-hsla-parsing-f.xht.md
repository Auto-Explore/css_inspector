# css/css-color/t425-hsla-parsing-f.xht

```json
{
  "format_version": 3,
  "file": "css/css-color/t425-hsla-parsing-f.xht"
}
```

## style[0]

```css
<![CDATA[
		html, body { background: white; }
		p { color: hsla(120, 100%, 25%, 1.0); }
		p { color: hsla(0, 100%, 25%, 1.0, 1.0); }
		p { color: hsla(0, 100%, 25%, 1.0,); }
		p { color: hsla(0, 255, 128, 1.0); }
		p { color: hsla(0%, 100%, 50%, 1.0); }
		p { color: hsla(0px, 100%, 50%, 1.0); }
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
