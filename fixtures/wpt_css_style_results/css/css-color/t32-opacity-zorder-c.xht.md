# css/css-color/t32-opacity-zorder-c.xht

```json
{
  "format_version": 3,
  "file": "css/css-color/t32-opacity-zorder-c.xht"
}
```

## style[0]

```css
<![CDATA[
		table { border-spacing: 2px; }
		td { border: 1px solid; }
		td, div { width: 10px; height: 10px; }
		div.up { margin-top: -10px; }
		div.pos { position: relative; }
		div.opc { opacity: 0.99; }
		div.red { background: red; }
		div.green { background: green; }
		.z0 { z-index: 0; }
		.z1 { z-index: 1; }
		.zm1 { z-index: -1; }
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
