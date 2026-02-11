# css/selectors/old-tests/css3-modsel-36.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-36.xml"
}
```

## style[0]

```css
<![CDATA[.red { background-color : red }
p:only-child { background-color : lime }
div.testText > div > p { margin-left : 1em }
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
