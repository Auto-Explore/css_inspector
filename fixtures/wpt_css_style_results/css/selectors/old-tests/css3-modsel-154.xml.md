# css/selectors/old-tests/css3-modsel-154.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-154.xml"
}
```

## style[0]

```css
<![CDATA[
  p { background: lime; }
  p, { background: red; }
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
