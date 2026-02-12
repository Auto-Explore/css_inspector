# css/selectors/old-tests/css3-modsel-74.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-74.xml"
}
```

## style[0]

```css
<![CDATA[.red { background-color : red }
ul > li:not(:nth-last-child(odd)) { background-color : lime }
ol > li:not(:nth-last-child(even)) { background-color : lime }
table.t1 tr:not(:nth-last-child(-n+4)) { background-color : lime }
table.t2 td:not(:nth-last-child(3n+1)) { background-color : lime }
table.t1 td, table.t2 td { border : thin black solid }
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
