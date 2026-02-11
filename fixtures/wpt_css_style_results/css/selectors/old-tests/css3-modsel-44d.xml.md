# css/selectors/old-tests/css3-modsel-44d.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-44d.xml"
}
```

## style[0]

```css
<![CDATA[
  #fail > div { background: red; }
  p { background: green; }
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
