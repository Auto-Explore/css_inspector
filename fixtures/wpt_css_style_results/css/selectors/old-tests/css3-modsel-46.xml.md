# css/selectors/old-tests/css3-modsel-46.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-46.xml"
}
```

## style[0]

```css
<![CDATA[.red { background-color : red }
div.stub > p ~ p { background-color : lime }]]>
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
