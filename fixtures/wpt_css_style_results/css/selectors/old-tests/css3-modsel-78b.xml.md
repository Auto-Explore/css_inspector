# css/selectors/old-tests/css3-modsel-78b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-78b.xml"
}
```

## style[0]

```css
<![CDATA[.green { background-color : lime ! important }
.t1 td:not(:last-child) { background-color : red }
p > *:not(:last-child) { background-color : red }
table.t1 td { border : thin black solid }
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
