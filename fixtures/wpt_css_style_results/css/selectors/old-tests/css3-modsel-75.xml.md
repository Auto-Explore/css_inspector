# css/selectors/old-tests/css3-modsel-75.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-75.xml"
}
```

## style[0]

```css
<![CDATA[.red { background-color : red }
p:not(:nth-of-type(3)) { background-color : lime }
dl > *:not(:nth-of-type(3n+1)) { background-color : lime }
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
