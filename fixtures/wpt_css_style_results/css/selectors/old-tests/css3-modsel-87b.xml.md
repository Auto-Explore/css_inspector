# css/selectors/old-tests/css3-modsel-87b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-87b.xml"
}
```

## style[0]

```css
<![CDATA[p { color: green ! important; }
blockquote + div ~ p { color: red; }]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
