# css/selectors/old-tests/css3-modsel-86.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-86.xml"
}
```

## style[0]

```css
<![CDATA[p { color: red; }
blockquote > div p { color: green; }
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
