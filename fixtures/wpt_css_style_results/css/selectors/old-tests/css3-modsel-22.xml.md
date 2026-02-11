# css/selectors/old-tests/css3-modsel-22.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-22.xml"
}
```

## style[0]

```css
<![CDATA[ul > li { background-color : red }
li:lang(en-GB) { background-color : lime }]]>
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
