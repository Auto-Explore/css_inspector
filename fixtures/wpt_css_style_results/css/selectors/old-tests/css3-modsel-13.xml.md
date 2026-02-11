# css/selectors/old-tests/css3-modsel-13.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-13.xml"
}
```

## style[0]

```css
<![CDATA[li { background-color : red }
.t1 { background-color : lime }
li.t2 { background-color : lime }
.t3 { background-color : red }]]>
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
