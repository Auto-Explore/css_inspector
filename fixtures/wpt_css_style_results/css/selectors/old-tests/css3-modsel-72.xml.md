# css/selectors/old-tests/css3-modsel-72.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-72.xml"
}
```

## style[0]

```css
<![CDATA[p:not(:root) { background-color: lime; }
div * { background-color: red; }
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
