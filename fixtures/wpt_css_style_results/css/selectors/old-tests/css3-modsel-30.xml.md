# css/selectors/old-tests/css3-modsel-30.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-30.xml"
}
```

## style[0]

```css
<![CDATA[.red { background-color : red }
p:nth-of-type(3) { background-color : lime }
dl > :nth-of-type(3n+1) { background-color : lime }
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
