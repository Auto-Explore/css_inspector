# css/selectors/old-tests/css3-modsel-109.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-109.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
@namespace html url(http://www.w3.org/1999/xhtml);
*|p, *|q, *|r, *|s, *|t{ display : block ; margin-bottom : 1em }
*|p, *|r, *|s { background-color : red }
*|*[*|title$="tait"] { background-color : lime }]]>
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
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
