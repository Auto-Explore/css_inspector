# css/selectors/old-tests/css3-modsel-167a.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-167a.xml"
}
```

## style[0]

```css
<![CDATA[
  p::first-line { background-color: red; }
  p:first-line { background-color: lime; }
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
