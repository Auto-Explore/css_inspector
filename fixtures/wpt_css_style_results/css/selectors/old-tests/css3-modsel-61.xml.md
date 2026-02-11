# css/selectors/old-tests/css3-modsel-61.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-61.xml"
}
```

## style[0]

```css
<![CDATA[div.stub > * { background-color : red }
div.stub *:not(:link) { background-color : lime }
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
