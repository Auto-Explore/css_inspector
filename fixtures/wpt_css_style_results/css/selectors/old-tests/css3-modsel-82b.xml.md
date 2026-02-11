# css/selectors/old-tests/css3-modsel-82b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-82b.xml"
}
```

## style[0]

```css
<![CDATA[.green { background-color : lime ! important }
.t1 *:not(:only-of-type) { background-color : red }
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
