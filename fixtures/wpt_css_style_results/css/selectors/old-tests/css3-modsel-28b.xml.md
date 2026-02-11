# css/selectors/old-tests/css3-modsel-28b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-28b.xml"
}
```

## style[0]

```css
<![CDATA[.green { background-color : lime ! important }
ul > li:nth-child(odd) { background-color : red }
ol > li:nth-child(even) { background-color : red }
table.t1 tr:nth-child(-n+4) { background-color : red }
table.t2 td:nth-child(3n+1) { background-color : red }]]>
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
