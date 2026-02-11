# css/selectors/old-tests/css3-modsel-171.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-171.xml"
}
```

## style[0]

```css
<![CDATA[
 p { color: green; }
 .fail { color: red; }
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
