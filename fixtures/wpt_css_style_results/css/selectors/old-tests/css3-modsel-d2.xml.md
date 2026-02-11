# css/selectors/old-tests/css3-modsel-d2.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-d2.xml"
}
```

## style[0]

```css
<![CDATA[
   #test { background: red; display: block; padding: 1em; }
   #stub ~ div div + div > div { background: lime; }
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
