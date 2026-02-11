# css/selectors/old-tests/css3-modsel-70.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-70.xml"
}
```

## style[0]

```css
<![CDATA[input, span { background-color : red }
input:not(:checked), input:not(:checked) + span { background-color : lime}]]>
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
