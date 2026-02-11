# css/selectors/old-tests/css3-modsel-73.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-73.xml"
}
```

## style[0]

```css
<![CDATA[.red { background-color : red }
ul > li:not(:nth-child(odd)) { background-color : lime }
ol > li:not(:nth-child(even)) { background-color : lime }
table.t1 tr:not(:nth-child(-n+4)) { background-color : lime }
table.t2 td:not(:nth-child(3n+1)) { background-color : lime }
table.t1 td, table.t2 td { border : thin black solid }]]>
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
