# css/selectors/old-tests/css3-modsel-81b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-81b.xml"
}
```

## style[0]

```css
<![CDATA[.green { background-color : lime ! important }
p:not(:only-child) { background-color : lime }
div.testText > div > p { margin-left : 1em }
]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
