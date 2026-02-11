# css/selectors/old-tests/css3-modsel-9.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-9.xml"
}
```

## style[0]

```css
<![CDATA[p { background-color : red }
p[title^="foo"] { background-color : lime }]]>
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
