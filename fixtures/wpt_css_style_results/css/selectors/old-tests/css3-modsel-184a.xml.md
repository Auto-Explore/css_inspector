# css/selectors/old-tests/css3-modsel-184a.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-184a.xml"
}
```

## style[0]

```css
<![CDATA[
p { color: lime; }
p[class$=""] { color: red; }
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
