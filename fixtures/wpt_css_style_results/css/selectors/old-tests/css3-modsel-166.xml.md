# css/selectors/old-tests/css3-modsel-166.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-166.xml"
}
```

## style[0]

```css
<![CDATA[
  p:first-letter { background-color: red; }
  p::first-letter { background-color: lime; }
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
