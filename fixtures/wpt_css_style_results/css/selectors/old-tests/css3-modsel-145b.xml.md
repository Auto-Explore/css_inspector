# css/selectors/old-tests/css3-modsel-145b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-145b.xml"
}
```

## style[0]

```css
<![CDATA[
  line { display: block; }
  [type~=odd] { background: lime ! important; }
  line:nth-of-type(odd) { background: red; }
  [hidden] { display: none; }
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
