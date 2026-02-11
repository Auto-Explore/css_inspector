# css/selectors/old-tests/css3-modsel-55.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-55.xml"
}
```

## style[0]

```css
<![CDATA[div.stub > * { color : red }
div.stub *:not([title$="tait"]) { color : lime }
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
