# css/selectors/old-tests/css3-modsel-63.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-63.xml"
}
```

## style[0]

```css
<![CDATA[div.stub * { color: lime; text-decoration: none; }
div.stub > * > *:not(:hover) { color: black }
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
