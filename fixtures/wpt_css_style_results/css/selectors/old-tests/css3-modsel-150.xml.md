# css/selectors/old-tests/css3-modsel-150.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-150.xml"
}
```

## style[0]

```css
<![CDATA[
 address:empty { background: lime; }
 address { background: red; margin: 0; height: 1em; }
 .text { margin: -1em 0 0 0; }
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
