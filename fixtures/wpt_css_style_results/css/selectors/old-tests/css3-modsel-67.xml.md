# css/selectors/old-tests/css3-modsel-67.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-67.xml"
}
```

## style[0]

```css
<![CDATA[div.stub * { background-color : red  }
div.stub *:not(:lang(fr)) { background-color : green }]]>
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
