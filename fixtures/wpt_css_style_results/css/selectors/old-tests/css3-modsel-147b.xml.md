# css/selectors/old-tests/css3-modsel-147b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-147b.xml"
}
```

## style[0]

```css
<![CDATA[
  line { display: block; }
  [type~=match] { background: lime ! important; }
  line:nth-last-of-type(3n-1) { background: red; }
  [hidden] { visibility: collapse; }
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
