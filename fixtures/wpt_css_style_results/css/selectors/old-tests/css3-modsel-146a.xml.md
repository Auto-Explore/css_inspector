# css/selectors/old-tests/css3-modsel-146a.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-146a.xml"
}
```

## style[0]

```css
<![CDATA[
  line { display: block; }
  [type~=match] { background: red; }
  line:nth-child(3n-1) { background: lime; }
  [hidden] { display: none; }
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
