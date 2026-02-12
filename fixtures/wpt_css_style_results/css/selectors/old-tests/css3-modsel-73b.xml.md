# css/selectors/old-tests/css3-modsel-73b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-73b.xml"
}
```

## style[0]

```css
<![CDATA[.green { background-color : lime ! important; }
ul > li:not(:nth-child(odd)) { background-color : red }
ol > li:not(:nth-child(even)) { background-color : red }
table.t1 tr:not(:nth-child(-n+4)) { background-color : red }
table.t2 td:not(:nth-child(3n+1)) { background-color : red }
table.t1 td, table.t2 td { border : thin black solid }]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
