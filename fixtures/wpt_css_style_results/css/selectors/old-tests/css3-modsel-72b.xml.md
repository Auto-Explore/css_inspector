# css/selectors/old-tests/css3-modsel-72b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-72b.xml"
}
```

## style[0]

```css
<![CDATA[html:not(:root), test:not(:root) { background-color: red; }
p { background-color: lime; }
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
