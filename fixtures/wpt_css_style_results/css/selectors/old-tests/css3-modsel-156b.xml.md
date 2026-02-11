# css/selectors/old-tests/css3-modsel-156b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-156b.xml"
}
```

## style[0]

```css
<![CDATA[
  foo % address, p { background: red; }
  p { background: lime; }
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
