# css/selectors/old-tests/css3-modsel-29.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-29.xml"
}
```

## style[0]

```css
<![CDATA[.red { background-color : red }
ul > li:nth-last-child(odd) { background-color : green }
ol > li:nth-last-child(even) { background-color : green }
table.t1 tr:nth-last-child(-n+4) { background-color : green }
table.t2 td:nth-last-child(3n+1) { background-color : green }]]>
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
