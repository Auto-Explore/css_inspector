# css/selectors/old-tests/css3-modsel-113.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-113.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
@namespace html url(http://www.w3.org/1999/xhtml);
*|p, *|address, *|q, *|r { display : block ; margin-bottom : 1em }
*|p, *|q { background-color : red }
*|*[|class~="foo"] { background-color : lime }]]>
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
