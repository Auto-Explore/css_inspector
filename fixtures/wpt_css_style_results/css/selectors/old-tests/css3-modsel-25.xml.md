# css/selectors/old-tests/css3-modsel-25.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-25.xml"
}
```

## style[0]

```css
<![CDATA[input, span { background-color : red }
input:checked, input:checked + span { background-color : lime}
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
