# css/selectors/old-tests/css3-modsel-37.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-37.xml"
}
```

## style[0]

```css
<![CDATA[.red { background-color : red }
.t1 :only-of-type { background-color : lime }
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
