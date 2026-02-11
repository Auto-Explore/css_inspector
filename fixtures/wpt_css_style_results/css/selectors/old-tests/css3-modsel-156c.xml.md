# css/selectors/old-tests/css3-modsel-156c.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-156c.xml"
}
```

## style[0]

```css
<![CDATA[
  foo % address, p { background: red ! important; }
  p { background: lime; }
]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
