# css/selectors/old-tests/css3-modsel-74b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-74b.xml"
}
```

## style[0]

```css
<![CDATA[.green { background-color : lime ! important; }
ul > li:not(:nth-last-child(odd)) { background-color : red }
ol > li:not(:nth-last-child(even)) { background-color : red }
table.t1 tr:not(:nth-last-child(-n+4)) { background-color : red }
table.t2 td:not(:nth-last-child(3n+1)) { background-color : red }
table.t1 td, table.t2 td { border : thin black solid }
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
