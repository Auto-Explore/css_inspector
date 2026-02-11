# css/selectors/old-tests/css3-modsel-33.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-33.xml"
}
```

## style[0]

```css
<![CDATA[.red { background-color : red }
.t1 td:last-child { background-color : lime }
p > *:last-child { background-color : lime }
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
