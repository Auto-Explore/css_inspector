# css/selectors/old-tests/css3-modsel-3.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-3.xml"
}
```

## style[0]

```css
<![CDATA[* { color : lime }
ul, p { color : red }
*.t1 { color : lime }
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
