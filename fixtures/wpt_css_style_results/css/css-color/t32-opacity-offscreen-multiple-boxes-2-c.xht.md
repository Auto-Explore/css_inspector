# css/css-color/t32-opacity-offscreen-multiple-boxes-2-c.xht

```json
{
  "format_version": 3,
  "file": "css/css-color/t32-opacity-offscreen-multiple-boxes-2-c.xht"
}
```

## style[0]

```css
<![CDATA[

		body { background: white; }

		div.test { margin: 1em; line-height: 0; font-family: monospace; }
		div.test span { opacity: 0.4; }
		div.test span a { background: blue; color: blue; }

		div.ref { height: 1em; width: 5em; background: rgb(153, 153, 255); }

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
