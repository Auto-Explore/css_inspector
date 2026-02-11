# css/selectors/old-tests/css3-modsel-64.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-64.xml"
}
```

## style[0]

```css
<![CDATA[div.stub * { color : lime }
div.stub > * > *:not(:active) { color : black }
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
