# css/selectors/old-tests/css3-modsel-151.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-151.xml"
}
```

## style[0]

```css
<![CDATA[
 address { background: lime; margin: 0; height: 1em; }
 address:empty { background: red; }
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
