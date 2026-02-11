# css/selectors/old-tests/css3-modsel-180a.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-180a.xml"
}
```

## style[0]

```css
<![CDATA[
  p { color: green; }
  p:first-letter { background: red; color: yellow; font-size: 4em; }
  p::first-letter { background: red; color: yellow; font-size: 4em; }
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
