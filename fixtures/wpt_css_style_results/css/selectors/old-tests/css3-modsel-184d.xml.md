# css/selectors/old-tests/css3-modsel-184d.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-184d.xml"
}
```

## style[0]

```css
<![CDATA[
p { color: red; }
p:not([class$=""]) { color: lime; }
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
