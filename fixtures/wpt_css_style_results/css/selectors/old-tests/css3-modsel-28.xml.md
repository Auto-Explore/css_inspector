# css/selectors/old-tests/css3-modsel-28.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-28.xml"
}
```

## style[0]

```css
<![CDATA[.red { background-color : red }
ul > li:nth-child(odd) { background-color : lime }
ol > li:nth-child(even) { background-color : lime }
table.t1 tr:nth-child(-n+4) { background-color : lime }
table.t2 td:nth-child(3n+1) { background-color : lime }]]>
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
