# css/selectors/old-tests/css3-modsel-32.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-32.xml"
}
```

## style[0]

```css
<![CDATA[.red { background-color : red }
.t1 td:first-child { background-color : lime }
p > *:first-child { background-color : lime }
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
