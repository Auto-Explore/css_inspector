# css/selectors/old-tests/css3-modsel-78.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-78.xml"
}
```

## style[0]

```css
<![CDATA[.red { background-color : red }
.t1 td:not(:last-child) { background-color : lime }
p > *:not(:last-child) { background-color : lime }
table.t1 td { border : thin black solid }
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
