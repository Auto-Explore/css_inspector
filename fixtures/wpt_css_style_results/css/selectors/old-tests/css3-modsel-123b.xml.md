# css/selectors/old-tests/css3-modsel-123b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-123b.xml"
}
```

## style[0]

```css
<![CDATA[@namespace html url(http://www.w3.org/1999/xhtml);
@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
div.stub > *|* { color : green ; display : block ;
                 margin-bottom : 1em }
div.stub > *|*:not(|*) { color : red ! important }
]]>
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
    {
      "message": "Stray declaration outside a rule.",
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
