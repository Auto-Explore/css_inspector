# css/css-color/t32-opacity-clamping-1.0-b.xht

```json
{
  "format_version": 3,
  "file": "css/css-color/t32-opacity-clamping-1.0-b.xht"
}
```

## style[0]

```css
<![CDATA[
			/* make sure clamped rather than a parser error */
			#two, #three, #four, #five, #six { opacity: 0.0; }

			#two { opacity: 1.0; }
			#three { opacity: 1.1; }
			#four { opacity: 1.9; }
			#five { opacity: 30; }
			#six { opacity: 7439.79; }
		]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
